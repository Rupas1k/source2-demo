#[cfg(feature = "dota")]
mod combat_log;
mod definition;
mod list;
mod value;

#[cfg(feature = "dota")]
pub use combat_log::*;
use definition::*;
pub use list::*;
pub use value::*;

use crate::error::GameEventError;
use crate::proto::CSvcMsgGameEvent;

pub struct GameEvent<'a> {
    id: i32,
    list: &'a GameEventList,
    keys: Vec<EventValue>,
}

impl<'a> GameEvent<'a> {
    pub fn new(list: &'a GameEventList, ge: CSvcMsgGameEvent) -> Self {
        let id = ge.eventid();
        let keys = ge
            .keys
            .iter()
            .map(|key| match key.r#type() {
                1 => EventValue::String(key.val_string().into()),
                2 => EventValue::Float(key.val_float()),
                3 => EventValue::Int(key.val_long()),
                4 => EventValue::Int(key.val_short()),
                5 => EventValue::Byte(key.val_byte() as u8),
                6 => EventValue::Bool(key.val_bool()),
                7 => EventValue::U64(key.val_uint64()),
                8 => EventValue::Int(key.val_long()),
                9 => EventValue::Int(key.val_short()),
                _ => unreachable!("Unknown event type: {}", key.r#type()),
            })
            .collect::<Vec<_>>();

        Self { id, list, keys }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.list.list[&self.id].name
    }

    pub fn iter(&self) -> impl Iterator<Item = (&str, &EventValue)> {
        self.keys
            .iter()
            .zip(self.list.list[&self.id].keys.iter())
            .map(|(value, key)| (key.name.as_str(), value))
    }

    pub fn get_value(&self, key: &str) -> Result<&EventValue, GameEventError> {
        let key = self.list.list[&self.id]
            .name_to_key
            .get(key)
            .ok_or_else(|| GameEventError::UnknownKey(key.to_string()))?;
        Ok(&self.keys[key.id as usize])
    }
}
