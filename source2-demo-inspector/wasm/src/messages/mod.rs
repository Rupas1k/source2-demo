mod changes;
mod events;
pub(crate) mod field_source;
mod fields;

use crate::input::ReplayBytes;
use prost::Message;
use prost_reflect::DynamicMessage;
use serde::Serialize;
use source2_demo::metadata::{MessageFamily, MessageSchema};
use source2_demo::proto::CDemoFileHeader;
use source2_demo::reader::{commands, packet_data, packet_messages, Command, RawPacketMessage};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Entry<'a> {
    command: usize,
    child: Option<usize>,
    id: u32,
    tick: i32,
    name: &'a str,
}

#[wasm_bindgen]
pub struct MessageInspector {
    file: ReplayBytes,
    commands: Vec<Command>,
    by_tick: std::collections::BTreeMap<i32, Vec<usize>>,
    schema: MessageSchema,
    packets: HashMap<usize, Vec<RawPacketMessage>>,
    payload_size: usize,
    fired_events: Option<Vec<events::FiredEvent>>,
    search_index: Option<Vec<[u32; 3]>>,
}

impl MessageInspector {
    pub(crate) fn new(file: ReplayBytes) -> Result<Self, String> {
        let commands = commands(&file).map_err(|e| e.to_string())?;
        let header = commands
            .first()
            .filter(|c| c.id == 1)
            .and_then(|c| CDemoFileHeader::decode(c.payload(&file).ok()?.as_ref()).ok());
        let schema = MessageSchema::new(header.as_ref().map_or("", |h| h.game_directory()))
            .map_err(|e| e.to_string())?;
        let mut by_tick = std::collections::BTreeMap::<i32, Vec<usize>>::new();
        for (ordinal, command) in commands.iter().enumerate() {
            by_tick
                .entry(command.tick.max(0))
                .or_default()
                .push(ordinal);
        }
        Ok(Self {
            by_tick,
            file,
            commands,
            schema,
            packets: HashMap::new(),
            payload_size: 0,
            search_index: None,
            fired_events: None,
        })
    }
}

#[wasm_bindgen]
impl MessageInspector {
    #[wasm_bindgen(js_name = "fromBytes")]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<MessageInspector, JsError> {
        Self::new(bytes.into()).map_err(|error| JsError::new(&error))
    }

    #[wasm_bindgen(js_name = "commandActivity")]
    pub fn command_activity(&self) -> Vec<u32> {
        self.by_tick
            .iter()
            .flat_map(|(&tick, commands)| [tick as u32, commands.len() as u32])
            .collect()
    }

    #[wasm_bindgen(js_name = "commandEntries")]
    pub fn command_entries(&self) -> Result<String, JsError> {
        let rows: Vec<_> = self
            .commands
            .iter()
            .enumerate()
            .map(|(command, entry)| Entry {
                command,
                child: None,
                id: entry.id,
                tick: entry.tick.max(0),
                name: self.schema.name(MessageFamily::Command, entry.id),
            })
            .collect();
        Ok(serde_json::to_string(&rows)?)
    }

    pub fn entries(&mut self, tick: i32) -> Result<String, JsError> {
        self.packets.clear();
        let mut rows = Vec::new();
        for &command in self.by_tick.get(&tick.max(0)).into_iter().flatten() {
            let c = &self.commands[command];
            rows.push(Entry {
                command,
                child: None,
                id: c.id,
                tick,
                name: self.schema.name(MessageFamily::Command, c.id),
            });
            if !c.is_packet() {
                continue;
            }
            let payload = c.payload(&self.file)?;
            let data = packet_data(c.id, &payload)?.unwrap_or_default();
            let messages = packet_messages(&data).collect::<Result<Vec<_>, _>>()?;
            for (child, m) in messages.iter().enumerate() {
                rows.push(Entry {
                    command,
                    child: Some(child),
                    id: m.id,
                    tick,
                    name: self.schema.name(MessageFamily::Packet, m.id),
                });
            }
            self.packets.insert(command, messages);
        }
        Ok(serde_json::to_string(&rows)?)
    }

    #[wasm_bindgen(js_name = "firedEvents")]
    pub fn fired_events(&mut self) -> Result<String, JsError> {
        if self.fired_events.is_none() {
            self.fired_events = Some(events::index(&self.file, &self.commands)?);
        }
        Ok(serde_json::to_string(&self.fired_events)?)
    }

    pub fn search(&mut self, query: &str) -> Result<Vec<u32>, JsError> {
        if self.search_index.is_none() {
            let mut index = Vec::new();
            for (ordinal, command) in self.commands.iter().enumerate() {
                index.push([ordinal as u32, u32::MAX, command.id]);
                if command.is_packet() {
                    let payload = command.payload(&self.file)?;
                    let data = packet_data(command.id, &payload)?.unwrap_or_default();
                    for (child, message) in packet_messages(&data).enumerate() {
                        index.push([ordinal as u32, child as u32, message?.id]);
                    }
                }
            }
            self.search_index = Some(index);
        }
        let query = query.to_lowercase();
        let id = query.parse::<u32>().ok();
        let mut matches = HashMap::new();
        let mut results = Vec::new();
        for &[command, child, wire_id] in self.search_index.iter().flatten() {
            let packet = child != u32::MAX;
            let matched = *matches.entry((packet, wire_id)).or_insert_with(|| {
                id == Some(wire_id)
                    || query.is_empty()
                    || self
                        .schema
                        .name(
                            if packet {
                                MessageFamily::Packet
                            } else {
                                MessageFamily::Command
                            },
                            wire_id,
                        )
                        .to_lowercase()
                        .contains(&query)
            });
            if matched {
                results.extend_from_slice(&[
                    command,
                    child,
                    wire_id,
                    self.commands[command as usize].tick.max(0) as u32,
                ]);
            }
        }
        Ok(results)
    }

    #[wasm_bindgen(js_name = "messageName")]
    pub fn message_name(&self, packet: bool, id: u32) -> String {
        let family = if packet {
            MessageFamily::Packet
        } else {
            MessageFamily::Command
        };
        self.schema.name(family, id).to_owned()
    }

    #[wasm_bindgen(js_name = "entityChanges")]
    pub fn entity_changes(&self, command: usize, child: usize) -> Result<String, JsError> {
        let changes =
            changes::changes(&self.file, command, child).map_err(|error| JsError::new(&error))?;
        Ok(serde_json::to_string(&changes)?)
    }

    #[wasm_bindgen(js_name = "stringTableChangeInfo")]
    pub fn string_table_change_info(
        &self,
        command: usize,
        child: usize,
    ) -> Result<String, JsError> {
        let table = changes::table_changes(&self.file, command, child)
            .map_err(|error| JsError::new(&error))?;
        Ok(serde_json::to_string(
            &(!table.rows.is_empty()).then_some((table.id, table.name)),
        )?)
    }

    #[wasm_bindgen(js_name = "stringTableChanges")]
    pub fn string_table_changes(
        &mut self,
        command: usize,
        child: usize,
    ) -> Result<String, JsError> {
        use fields::Field;

        let table = changes::table_changes(&self.file, command, child)
            .map_err(|error| JsError::new(&error))?;
        let rows = table
            .rows
            .iter()
            .map(|row| {
                let mut fields = vec![Field::text("str", "string", row.key().to_owned())];
                if let Some(value) = row.value() {
                    fields.push(Field::bytes("data", value.len()));
                }
                Field::group(&row.index().to_string(), fields)
            })
            .collect();
        let table = Field::group(
            "table",
            vec![
                Field::text("table_name", "string", table.name),
                Field::text("table_id", "int32", table.id.to_string()),
                Field::group("items", rows),
            ],
        );
        Ok(serde_json::to_string(&vec![table])?)
    }

    #[wasm_bindgen(js_name = "payloadSize")]
    pub fn payload_size(&self) -> usize {
        self.payload_size
    }

    pub fn inspect(&mut self, command: usize, child: Option<usize>) -> Result<String, JsError> {
        self.payload_size = 0;
        let c = self
            .commands
            .get(command)
            .ok_or_else(|| JsError::new("Invalid demo command"))?;
        let (id, payload) = if let Some(i) = child {
            let m = self
                .packets
                .get(&command)
                .and_then(|p| p.get(i))
                .ok_or_else(|| JsError::new("Invalid packet message"))?;
            (m.id, std::borrow::Cow::Borrowed(m.payload.as_slice()))
        } else {
            (c.id, c.payload(&self.file)?)
        };
        self.payload_size = payload.len();
        let rows = match self.schema.descriptor(
            if child.is_none() {
                MessageFamily::Command
            } else {
                MessageFamily::Packet
            },
            id,
        ) {
            Some(desc) => match DynamicMessage::decode(desc, payload.as_ref()) {
                Ok(message) => fields::fields(&message),
                Err(_) => vec![fields::Field::bytes("Undecoded payload", payload.len())],
            },
            None => vec![fields::Field::bytes("Payload", payload.len())],
        };
        Ok(serde_json::to_string(&rows)?)
    }
}
