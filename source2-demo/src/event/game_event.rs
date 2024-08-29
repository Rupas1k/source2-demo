use crate::error::GameEventError;
use crate::proto::{CSvcMsgGameEvent, CSvcMsgGameEventList};
use hashbrown::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum EventValue {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
    Byte(u8),
    U64(u64),
}

#[derive(Default)]
pub struct GameEventList {
    pub(crate) list: HashMap<i32, Rc<GameEventDefinition>>,
    // name_to_definition: HashMap<String, Rc<GameEventDefinition>>,
}

impl GameEventList {
    pub fn new(list: CSvcMsgGameEventList) -> Self {
        // let mut name_to_definition = HashMap::default();
        let list = list
            .descriptors
            .into_iter()
            .map(|descriptor| {
                let keys = descriptor
                    .keys
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(i, key)| {
                        Rc::new(GameEventKey {
                            id: i as i32,
                            name: key.name().into(),
                        })
                    })
                    .collect::<Vec<_>>();
                let definition = Rc::new(GameEventDefinition {
                    name: descriptor.name().into(),
                    keys: keys.clone(),
                    name_to_key: keys
                        .into_iter()
                        .map(|key| (key.name.clone(), key))
                        .collect(),
                });
                // name_to_definition.insert(descriptor.name().to_string(), definition.clone());
                (descriptor.eventid(), definition)
            })
            .collect::<HashMap<_, _>>();

        Self {
            list,
            // name_to_definition,
        }
    }
}

pub(crate) struct GameEventDefinition {
    pub(crate) name: String,
    keys: Vec<Rc<GameEventKey>>,
    name_to_key: HashMap<String, Rc<GameEventKey>>,
}

#[derive(Debug)]
pub struct GameEventKey {
    id: i32,
    name: String,
}

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

impl TryInto<String> for EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<String, GameEventError> {
        if let EventValue::String(x) = self {
            Ok(x)
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<String> for &EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<String, GameEventError> {
        if let EventValue::String(x) = self {
            Ok(x.to_owned())
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<bool> for EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<bool, GameEventError> {
        if let EventValue::Bool(x) = self {
            Ok(x)
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<bool> for &EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<bool, GameEventError> {
        if let EventValue::Bool(x) = self {
            Ok(*x)
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<f32> for EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<f32, GameEventError> {
        if let EventValue::Float(x) = self {
            Ok(x)
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "f32".to_string(),
            ))
        }
    }
}

impl TryInto<f32> for &EventValue {
    type Error = GameEventError;

    fn try_into(self) -> Result<f32, GameEventError> {
        if let EventValue::Float(x) = self {
            Ok(*x)
        } else {
            Err(GameEventError::ConversionError(
                format!("{:?}", self),
                "f32".to_string(),
            ))
        }
    }
}

macro_rules! impl_try_into_for_integers {
    ($target:ty) => {
        impl TryInto<$target> for EventValue {
            type Error = GameEventError;

            fn try_into(self) -> Result<$target, GameEventError> {
                match self {
                    EventValue::Int(x) => Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::Byte(x) => Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::U64(x) => Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::Float(x) => Ok(x as $target),
                    _ => Err(GameEventError::ConversionError(
                        format!("{:?}", self),
                        stringify!($target).to_string(),
                    )),
                }
            }
        }

        impl TryInto<$target> for &EventValue {
            type Error = GameEventError;

            fn try_into(self) -> Result<$target, GameEventError> {
                match self {
                    EventValue::Int(x) => Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", *x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::Byte(x) => Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::U64(x) => Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                        GameEventError::ConversionError(
                            format!("{:?}", *x),
                            stringify!($target).to_string(),
                        )
                    })?),
                    EventValue::Float(x) => Ok(*x as $target),
                    _ => Err(GameEventError::ConversionError(
                        format!("{:?}", self),
                        stringify!($target).to_string(),
                    )),
                }
            }
        }
    };
}

impl_try_into_for_integers!(i8);
impl_try_into_for_integers!(i16);
impl_try_into_for_integers!(i32);
impl_try_into_for_integers!(i64);
impl_try_into_for_integers!(i128);
impl_try_into_for_integers!(u8);
impl_try_into_for_integers!(u16);
impl_try_into_for_integers!(u32);
impl_try_into_for_integers!(u64);
impl_try_into_for_integers!(u128);
impl_try_into_for_integers!(usize);
impl_try_into_for_integers!(isize);
