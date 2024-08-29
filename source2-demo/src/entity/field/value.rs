use crate::error::FieldValueError;

/// Special type for [`Entity`](crate::Entity) field value that can be converted
/// into Rust type using `try_into`.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    Boolean(bool),
    String(String),
    Float(f32),

    Vector2D([f32; 2]),
    Vector3D([f32; 3]),
    Vector4D([f32; 4]),

    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),

    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
}

impl TryInto<String> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<String, FieldValueError> {
        if let FieldValue::String(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<String> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<String, FieldValueError> {
        if let FieldValue::String(x) = self {
            Ok(x.to_owned())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "String".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 2]> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 2], FieldValueError> {
        if let FieldValue::Vector2D(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 2]".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 2]> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 2], FieldValueError> {
        if let FieldValue::Vector2D(x) = self {
            Ok(*x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 2]".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32)> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32), FieldValueError> {
        if let FieldValue::Vector2D(x) = self {
            Ok(x.into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32)> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32), FieldValueError> {
        if let FieldValue::Vector2D(x) = self {
            Ok((*x).into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 3]> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 3], FieldValueError> {
        if let FieldValue::Vector3D(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 3]".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 3]> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 3], FieldValueError> {
        if let FieldValue::Vector3D(x) = self {
            Ok(*x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 3]".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32, f32)> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32, f32), FieldValueError> {
        if let FieldValue::Vector3D(x) = self {
            Ok(x.into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32, f32)> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32, f32), FieldValueError> {
        if let FieldValue::Vector3D(x) = self {
            Ok((*x).into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 4]> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 4], FieldValueError> {
        if let FieldValue::Vector4D(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 4]".to_string(),
            ))
        }
    }
}

impl TryInto<[f32; 4]> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<[f32; 4], FieldValueError> {
        if let FieldValue::Vector4D(x) = self {
            Ok(*x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "[f32; 4]".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32, f32, f32)> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32, f32, f32), FieldValueError> {
        if let FieldValue::Vector4D(x) = self {
            Ok(x.into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32, f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<(f32, f32, f32, f32)> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<(f32, f32, f32, f32), FieldValueError> {
        if let FieldValue::Vector4D(x) = self {
            Ok((*x).into())
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "(f32, f32, f32, f32)".to_string(),
            ))
        }
    }
}

impl TryInto<Vec<f32>> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<Vec<f32>, FieldValueError> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "Vec<f32>".to_string(),
            )),
        }
    }
}

impl TryInto<Vec<f32>> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<Vec<f32>, FieldValueError> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "Vec<f32>".to_string(),
            )),
        }
    }
}

impl TryInto<f32> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<f32, FieldValueError> {
        if let FieldValue::Float(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "f32".to_string(),
            ))
        }
    }
}

impl TryInto<f32> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<f32, FieldValueError> {
        if let FieldValue::Float(x) = self {
            Ok(*x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "f32".to_string(),
            ))
        }
    }
}

impl TryInto<bool> for FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<bool, FieldValueError> {
        if let FieldValue::Boolean(x) = self {
            Ok(x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "bool".to_string(),
            ))
        }
    }
}

impl TryInto<bool> for &FieldValue {
    type Error = FieldValueError;

    fn try_into(self) -> Result<bool, FieldValueError> {
        if let FieldValue::Boolean(x) = self {
            Ok(*x)
        } else {
            Err(FieldValueError::ConversionError(
                format!("{:?}", self),
                "bool".to_string(),
            ))
        }
    }
}

macro_rules! impl_try_into_for_integers {
    ($target:ty) => {
        impl TryInto<$target> for FieldValue {
            type Error = FieldValueError;

            fn try_into(self) -> Result<$target, FieldValueError> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok((x == 1) as $target),
                    FieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Float(x) => Ok(x as $target),
                    _ => Err(FieldValueError::ConversionError(
                        format!("{:?}", self),
                        stringify!($target).to_string(),
                    )),
                }
            }
        }

        impl TryInto<$target> for &FieldValue {
            type Error = FieldValueError;

            fn try_into(self) -> Result<$target, FieldValueError> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok(x == 1 as $target),
                    FieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            FieldValueError::ConversionError(
                                format!("{:?}", x),
                                stringify!($target).to_string(),
                            )
                        })?)
                    }
                    FieldValue::Float(x) => Ok(*x as $target),
                    _ => Err(FieldValueError::ConversionError(
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

#[allow(dead_code)]
impl FieldValue {
    #[inline]
    pub(crate) fn as_string(&self) -> String {
        if let FieldValue::String(s) = self {
            s.to_string()
        } else {
            panic!("Tried to read as String, Found {:?}", self);
        }
    }

    #[inline]
    pub(crate) fn as_bool(&self) -> bool {
        if let FieldValue::Boolean(b) = self {
            *b
        } else {
            panic!("Tried to read as Boolean, Found {:?}", self);
        }
    }

    #[inline]
    pub(crate) fn as_float(&self) -> f32 {
        if let FieldValue::Float(f) = self {
            *f
        } else {
            panic!("Tried to read as Float, Found {:?}", self);
        }
    }

    #[inline]
    pub(crate) fn as_vector2d(&self) -> &[f32; 2] {
        if let FieldValue::Vector2D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector2D, Found {:?}", self);
        }
    }

    #[inline]
    pub(crate) fn as_vector(&self) -> &[f32; 3] {
        if let FieldValue::Vector3D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector3D, Found {:?}", self);
        }
    }

    #[inline]
    pub(crate) fn as_vector4d(&self) -> &[f32; 4] {
        if let FieldValue::Vector4D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector4D, Found {:?}", self);
        }
    }
}
