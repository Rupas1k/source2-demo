use crate::error::GameEventError;

#[derive(Debug)]
pub enum EventValue {
    String(String),
    Float(f32),
    Int(i32),
    Bool(bool),
    Byte(u8),
    U64(u64),
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
