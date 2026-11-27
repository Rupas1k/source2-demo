use prost::Message;
use serde::Serialize;
use source2_demo::error::ParserError;
use source2_demo::proto::{CSvcMsgGameEvent, CSvcMsgGameEventList, EBaseGameEvents};
use source2_demo::reader::{packet_data, packet_messages, Command};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct FiredEvent {
    id: i32,
    name: String,
    tick: i32,
    command: usize,
    child: usize,
    fields: Vec<EventField>,
}

#[derive(Serialize)]
struct EventField {
    name: String,
    kind: &'static str,
    value: String,
}

pub fn index(file: &[u8], commands: &[Command]) -> Result<Vec<FiredEvent>, ParserError> {
    let mut definitions = HashMap::new();
    let mut events = Vec::new();
    for (ordinal, command) in commands
        .iter()
        .enumerate()
        .filter(|(_, command)| command.is_packet())
    {
        let payload = command.payload(file)?;
        let Some(data) = packet_data(command.id, &payload)? else {
            continue;
        };
        for (child, message) in packet_messages(&data).enumerate() {
            let message = message?;
            match EBaseGameEvents::try_from(message.id as i32) {
                Ok(EBaseGameEvents::GeSource1LegacyGameEventList) => {
                    definitions = CSvcMsgGameEventList::decode(message.payload.as_slice())?
                        .descriptors
                        .into_iter()
                        .map(|definition| (definition.eventid(), definition))
                        .collect();
                }
                Ok(EBaseGameEvents::GeSource1LegacyGameEvent) => {
                    let event = CSvcMsgGameEvent::decode(message.payload.as_slice())?;
                    let id = event.eventid();
                    let definition = definitions.get(&id);
                    let name = if !event.event_name().is_empty() {
                        event.event_name().to_owned()
                    } else {
                        definition
                            .map(|definition| definition.name().to_owned())
                            .unwrap_or_else(|| format!("Event {id}"))
                    };
                    let fields = event
                        .keys
                        .iter()
                        .enumerate()
                        .map(|(index, key)| {
                            let (kind, value) = match key.r#type() {
                                1 => (
                                    "string",
                                    String::from_utf8_lossy(key.val_string()).into_owned(),
                                ),
                                2 => ("float", key.val_float().to_string()),
                                3 => ("long", key.val_long().to_string()),
                                4 => ("short", key.val_short().to_string()),
                                5 => ("byte", (key.val_byte() as u8).to_string()),
                                6 => ("bool", key.val_bool().to_string()),
                                7 => ("uint64", key.val_uint64().to_string()),
                                8 => ("int", key.val_long().to_string()),
                                9 => ("short", key.val_short().to_string()),
                                _ => ("unknown", format!("{key:?}")),
                            };
                            let name = definition
                                .and_then(|definition| definition.keys.get(index))
                                .filter(|key| !key.name().is_empty())
                                .map(|key| key.name().to_owned())
                                .unwrap_or_else(|| index.to_string());
                            EventField { name, kind, value }
                        })
                        .collect();
                    events.push(FiredEvent {
                        id,
                        name,
                        tick: command.tick.max(0),
                        command: ordinal,
                        child,
                        fields,
                    });
                }
                _ => {}
            }
        }
    }
    Ok(events)
}
