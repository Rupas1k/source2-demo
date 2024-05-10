use anyhow::{anyhow, bail, format_err};
use std::fmt::{Display, Formatter};

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

impl Display for FieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldValue::Boolean(val) => write!(f, "{}", val),
            FieldValue::String(val) => write!(f, "{}", val),
            FieldValue::Float(val) => write!(f, "{}", val),
            FieldValue::Vector2D(val) => write!(f, "[{}, {}]", val[0], val[1]),
            FieldValue::Vector3D(val) => write!(f, "[{}, {}, {}]", val[0], val[1], val[2]),
            FieldValue::Vector4D(val) => {
                write!(f, "[{}, {}, {}, {}]", val[0], val[1], val[2], val[3])
            }
            FieldValue::Signed8(val) => write!(f, "{}", val),
            FieldValue::Signed16(val) => write!(f, "{}", val),
            FieldValue::Signed32(val) => write!(f, "{}", val),
            FieldValue::Signed64(val) => write!(f, "{}", val),
            FieldValue::Unsigned8(val) => write!(f, "{}", val),
            FieldValue::Unsigned16(val) => write!(f, "{}", val),
            FieldValue::Unsigned32(val) => write!(f, "{}", val),
            FieldValue::Unsigned64(val) => write!(f, "{}", val),
        }
    }
}

impl TryInto<String> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<String, anyhow::Error> {
        if let FieldValue::String(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into String", self)
        }
    }
}

impl TryInto<String> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<String, anyhow::Error> {
        if let FieldValue::String(x) = self {
            Ok(x.to_owned())
        } else {
            bail!("Error converting \"{}\" into String", self)
        }
    }
}

impl TryInto<[f32; 2]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 2], anyhow::Error> {
        if let FieldValue::Vector2D(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into [f32; 2]", self)
        }
    }
}

impl TryInto<[f32; 2]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 2], anyhow::Error> {
        if let FieldValue::Vector2D(x) = self {
            Ok(*x)
        } else {
            bail!("Error converting \"{}\" into [f32; 2]", self)
        }
    }
}

impl TryInto<(f32, f32)> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32), anyhow::Error> {
        if let FieldValue::Vector2D(x) = self {
            Ok(x.into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32))", self)
        }
    }
}

impl TryInto<(f32, f32)> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32), anyhow::Error> {
        if let FieldValue::Vector2D(x) = self {
            Ok((*x).into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32))", self)
        }
    }
}

impl TryInto<[f32; 3]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 3], anyhow::Error> {
        if let FieldValue::Vector3D(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into [f32; 3]", self)
        }
    }
}

impl TryInto<[f32; 3]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 3], anyhow::Error> {
        if let FieldValue::Vector3D(x) = self {
            Ok(*x)
        } else {
            bail!("Error converting \"{}\" into [f32; 3]", self)
        }
    }
}

impl TryInto<(f32, f32, f32)> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32, f32), anyhow::Error> {
        if let FieldValue::Vector3D(x) = self {
            Ok(x.into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32, f32)", self)
        }
    }
}

impl TryInto<(f32, f32, f32)> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32, f32), anyhow::Error> {
        if let FieldValue::Vector3D(x) = self {
            Ok((*x).into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32, f32)", self)
        }
    }
}

impl TryInto<[f32; 4]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 4], anyhow::Error> {
        if let FieldValue::Vector4D(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into [f32; 4]", self)
        }
    }
}

impl TryInto<[f32; 4]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<[f32; 4], anyhow::Error> {
        if let FieldValue::Vector4D(x) = self {
            Ok(*x)
        } else {
            bail!("Error converting \"{}\" into [f32; 4]", self)
        }
    }
}

impl TryInto<(f32, f32, f32, f32)> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32, f32, f32), anyhow::Error> {
        if let FieldValue::Vector4D(x) = self {
            Ok(x.into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32, f32, f32)", self)
        }
    }
}

impl TryInto<(f32, f32, f32, f32)> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<(f32, f32, f32, f32), anyhow::Error> {
        if let FieldValue::Vector4D(x) = self {
            Ok((*x).into())
        } else {
            bail!("Error converting \"{}\" into (f32, f32, f32, f32)", self)
        }
    }
}

impl TryInto<Vec<f32>> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<f32>, anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => bail!("Error converting \"{}\" into Vec<f32>", self),
        }
    }
}

impl TryInto<Vec<f32>> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<Vec<f32>, anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => bail!("Error converting \"{}\" into Vec<f32>", self),
        }
    }
}

impl TryInto<f32> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<f32, anyhow::Error> {
        if let FieldValue::Float(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into f32", self)
        }
    }
}

impl TryInto<f32> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<f32, anyhow::Error> {
        if let FieldValue::Float(x) = self {
            Ok(*x)
        } else {
            bail!("Error converting \"{}\" into f32", self)
        }
    }
}

impl TryInto<bool> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<bool, anyhow::Error> {
        if let FieldValue::Boolean(x) = self {
            Ok(x)
        } else {
            bail!("Error converting \"{}\" into bool", self)
        }
    }
}

impl TryInto<bool> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> anyhow::Result<bool, anyhow::Error> {
        if let FieldValue::Boolean(x) = self {
            Ok(*x)
        } else {
            bail!("Error converting \"{}\" into bool", self)
        }
    }
}

macro_rules! impl_try_into_for_integers {
    ($target:ty) => {
        impl TryInto<$target> for FieldValue {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<$target, anyhow::Error> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok((x == 1) as $target),
                    FieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Float(x) => Ok(x as $target),
                    _ => Err(anyhow!(
                        "Cannot convert \"{}\" into {}",
                        self,
                        stringify!($target)
                    )),
                }
            }
        }

        impl TryInto<$target> for &FieldValue {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<$target, anyhow::Error> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok(x == 1 as $target),
                    FieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            anyhow!("Error converting \"{x}\" into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Float(x) => Ok(*x as $target),
                    _ => bail!("Cannot convert \"{}\" into {}", self, stringify!($target)),
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

impl FieldValue {
    pub fn as_str(&self) -> &str {
        if let FieldValue::String(s) = self {
            s.as_str()
        } else {
            panic!("Tried to read as String, Found {:?}", self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let FieldValue::Boolean(b) = self {
            *b
        } else {
            panic!("Tried to read as Boolean, Found {:?}", self);
        }
    }

    pub fn as_float(&self) -> f32 {
        if let FieldValue::Float(f) = self {
            *f
        } else {
            panic!("Tried to read as Float, Found {:?}", self);
        }
    }

    pub fn as_vector2d(&self) -> &[f32; 2] {
        if let FieldValue::Vector2D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector2D, Found {:?}", self);
        }
    }

    pub fn as_vector3d(&self) -> &[f32; 3] {
        if let FieldValue::Vector3D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector3D, Found {:?}", self);
        }
    }

    pub fn as_vector4d(&self) -> &[f32; 4] {
        if let FieldValue::Vector4D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector4D, Found {:?}", self);
        }
    }
}
