use crate::records::*;
use crate::MessageInspector;
use std::fmt::Display;
use std::io::Cursor;
use std::rc::Rc;

use source2_demo::prelude::*;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsError;

#[wasm_bindgen]
pub struct ReplaySession {
    parser: Parser<'static, source2_demo::reader::SeekableReader<Cursor<Rc<[u8]>>>>,
    file: Rc<[u8]>,
}

#[derive(Default)]
struct InspectorState;

#[observer]
#[uses_entities]
#[uses_string_tables]
#[uses_game_events]
impl InspectorState {}

#[wasm_bindgen]
impl ReplaySession {
    #[wasm_bindgen(constructor, js_name = "fromBytes")]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<ReplaySession, JsError> {
        if bytes.len() < 16 || !bytes.starts_with(b"PBDEMS2\0") {
            return Err(JsError::new("Expected a Source 2 demo header."));
        }
        let file: Rc<[u8]> = bytes.into();
        let cursor = Cursor::new(file.clone());
        let mut parser = Parser::from_reader(cursor).map_err(to_js_error)?;
        parser.register_observer::<InspectorState>();
        Ok(Self { parser, file })
    }

    #[wasm_bindgen(js_name = "openMessages")]
    pub fn open_messages(&self) -> Result<MessageInspector, JsError> {
        MessageInspector::new(self.file.clone()).map_err(to_js_error)
    }

    #[wasm_bindgen(js_name = "findFieldSource")]
    pub fn find_field_source(&self, index: usize, path: Vec<u16>) -> Result<String, JsError> {
        let entity = self
            .parser
            .context()
            .entities()
            .get_by_index(index)
            .map_err(to_js_error)?;
        let fields = entity.fields();
        let Some(value) = fields
            .iter()
            .find(|field| field.path == path)
            .and_then(|field| field.value)
        else {
            return Ok("null".into());
        };
        let commands = source2_demo::reader::commands(&self.file)?;
        let source = crate::messages::field_source::find(
            &self.file,
            &commands,
            self.parser.replay_position(),
            entity.handle(),
            path,
            value,
        )
        .map_err(to_js_error)?;
        Ok(serde_json::to_string(&source)?)
    }

    #[wasm_bindgen(js_name = "recordedTicks")]
    pub fn recorded_ticks(&self) -> Result<Vec<i32>, JsError> {
        let mut ticks: Vec<_> = source2_demo::reader::commands(&self.file)?
            .into_iter()
            .map(|command| command.tick.max(0))
            .collect();
        ticks.sort_unstable();
        ticks.dedup();
        Ok(ticks)
    }

    #[wasm_bindgen(js_name = "tick")]
    pub fn tick(&self) -> i32 {
        self.parser.context().tick() as i32
    }

    #[wasm_bindgen(js_name = "totalTicks")]
    pub fn total_ticks(&self) -> i32 {
        self.parser.replay_info().playback_ticks()
    }

    #[wasm_bindgen(js_name = "runToTick")]
    pub fn run_to_tick(&mut self, tick: i32) -> Result<(), JsError> {
        let target_tick = tick.max(0) as u32;
        let current_tick = self.parser.context().tick();

        if target_tick == current_tick {
            return Ok(());
        }

        self.parser.jump_to_tick(target_tick).map_err(to_js_error)
    }

    #[wasm_bindgen(js_name = "adjacentTick")]
    pub fn adjacent_tick(&self, forward: bool) -> i32 {
        let tick = self.tick().max(0);
        if forward {
            return tick.saturating_add(1).min(self.total_ticks());
        }
        let previous = self.parser.context().previous_tick();
        if previous < tick as u32 {
            previous as i32
        } else {
            tick.saturating_sub(1).max(0)
        }
    }

    #[wasm_bindgen(js_name = "entityHandle")]
    pub fn entity_handle(&self, index: usize) -> Option<u32> {
        self.parser
            .context()
            .entities()
            .get_by_index(index)
            .ok()
            .map(Entity::handle)
    }

    #[wasm_bindgen(js_name = "entityIndexForHandle")]
    pub fn entity_index_for_handle(&self, handle: u32) -> Option<u32> {
        self.parser
            .context()
            .entities()
            .get_by_handle(handle as usize)
            .ok()
            .filter(|entity| entity.handle() == handle)
            .map(Entity::index)
    }

    #[wasm_bindgen(js_name = "listEntities")]
    pub fn list_entities(&self) -> Vec<EntityRecord> {
        self.parser
            .context()
            .entities()
            .iter()
            .map(entity_li)
            .collect()
    }

    #[wasm_bindgen(js_name = "listBaselineEntities")]
    pub fn list_baseline_entities(&self) -> Vec<EntityRecord> {
        self.parser
            .context()
            .baseline_entities()
            .into_iter()
            .map(|entity| EntityRecord {
                index: entity.class_id,
                name: entity.class_name,
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "listEntityFields")]
    pub fn list_entity_fields(&self, entity_index: i32) -> Option<Vec<EntityFieldRecord>> {
        self.parser
            .context()
            .entities()
            .get_by_index(entity_index as usize)
            .ok()
            .map(|entity| collect_entity_field_list(entity.fields()))
    }

    #[wasm_bindgen(js_name = "listBaselineEntityFields")]
    pub fn list_baseline_entity_fields(&self, entity_index: i32) -> Option<Vec<EntityFieldRecord>> {
        self.parser
            .context()
            .baseline_fields(entity_index)
            .map(collect_entity_field_list)
    }

    #[wasm_bindgen(js_name = "listStringTables")]
    pub fn list_string_tables(&self) -> Vec<TableRecord> {
        self.parser
            .context()
            .string_tables()
            .iter()
            .map(|string_table| TableRecord {
                index: string_table.index(),
                name: string_table.name().to_string(),
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "listStringTableItems")]
    pub fn list_string_table_items(
        &self,
        string_table_name: String,
    ) -> Option<Vec<TableEntryRecord>> {
        self.parser
            .context()
            .string_tables()
            .get_by_name(&string_table_name)
            .ok()
            .map(|string_table| {
                string_table
                    .iter()
                    .map(|row| TableEntryRecord {
                        index: row.index(),
                        string: Some(row.key().as_bytes().to_vec()),
                        user_data: row.value().map(|value| value.to_vec()),
                    })
                    .collect()
            })
    }

    #[wasm_bindgen(js_name = "listGameEvents")]
    pub fn list_game_events(&self) -> Vec<EventRecord> {
        let mut events = self
            .parser
            .context()
            .game_events()
            .iter()
            .map(|(id, definition)| EventRecord {
                id,
                name: definition.name().to_string(),
            })
            .collect::<Vec<_>>();
        events.sort_by_key(|event| event.id);
        events
    }

    #[wasm_bindgen(js_name = "listGameEventKeys")]
    pub fn list_game_event_keys(&self, event_id: i32) -> Option<Vec<EventKeyRecord>> {
        self.parser
            .context()
            .game_events()
            .iter()
            .find(|(id, _definition)| *id == event_id)
            .map(|(_id, definition)| {
                definition
                    .keys()
                    .map(|key| EventKeyRecord {
                        index: key.id(),
                        name: key.name().to_string(),
                        type_id: key.type_id(),
                        type_name: key.type_name().to_string(),
                    })
                    .collect()
            })
    }
}

fn entity_li(entity: &Entity) -> EntityRecord {
    EntityRecord {
        index: entity.index() as i32,
        name: entity.class().name().to_string(),
    }
}

fn collect_entity_field_list(fields: Vec<EntityField<'_>>) -> Vec<EntityFieldRecord> {
    fields
        .into_iter()
        .map(|field| EntityFieldRecord {
            path: field.path.into_iter().collect(),
            named_path: field.name.split('.').map(ToString::to_string).collect(),
            value: field.value.map(format_field_value).unwrap_or_default(),
            encoded_as: field.field_type,
            decoded_as: field.decoded_type.unwrap_or("None").to_string(),
        })
        .collect()
}

pub(crate) fn format_field_value(value: &FieldValue) -> String {
    match value {
        FieldValue::String(value) => value.clone(),
        other => other.to_string(),
    }
}

fn to_js_error(error: impl Display) -> JsError {
    JsError::new(&error.to_string())
}

#[wasm_bindgen(js_name = "isEHandleValid")]
pub fn is_ehandle_valid(handle: u32) -> bool {
    handle != u32::MAX && handle != 0x00ff_ffff
}

#[wasm_bindgen(js_name = "eHandleToIndex")]
pub fn ehandle_to_index(handle: u32) -> i32 {
    (handle & 0x3fff) as i32
}
