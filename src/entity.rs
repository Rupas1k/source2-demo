use crate::class::Class;
use crate::field::FieldPath;
use crate::field::FieldState;
use anyhow::{anyhow, format_err, Result};
use nohash_hasher::IntMap;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityAction {
    Created = 1 << 0,
    Updated = 1 << 1,
    Deleted = 1 << 2,
    Entered = 1 << 3,
    Left = 1 << 4,
}

pub struct Entities {
    pub(crate) entity_full_packets: u32,
    pub(crate) undone_entities: VecDeque<(i32, isize)>,
    pub(crate) index_to_entity: IntMap<i32, Entity>,
}

impl Entities {
    pub(crate) fn new() -> Self {
        Entities {
            index_to_entity: IntMap::default(),
            undone_entities: VecDeque::new(),
            entity_full_packets: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.index_to_entity.values()
    }

    pub fn get_by_index(&self, index: &i32) -> Result<&Entity> {
        self.index_to_entity
            .get(index)
            .ok_or_else(|| anyhow!("No entities for index {index}"))
    }
    pub fn get_by_handle(&self, handle: &i32) -> Result<&Entity> {
        self.get_by_index(&(handle & 0x3fff))
    }
    pub fn get_by_class_id(&self, id: &i32) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| &entity.class.id == id)
            .ok_or_else(|| anyhow!("No entities for class with id {id}"))
    }
    pub fn get_by_class_name(&self, name: &str) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| entity.class.name.as_ref() == name)
            .ok_or_else(|| anyhow!("No entities for class with name {name}"))
    }

    pub fn get_all_by_class_id<'a>(&'a self, id: &'a i32) -> impl Iterator<Item = &Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| entity.class.id == *id)
    }

    pub fn get_all_by_class_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| entity.class.name == name.into())
    }

    pub fn get_all_by_predicate<'a>(
        &'a self,
        predicate: &'a dyn Fn(&Entity) -> bool,
    ) -> impl Iterator<Item = &Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| predicate(entity))
    }
}

#[derive(Clone)]
pub struct Entity {
    pub(crate) index: i32,
    pub(crate) serial: i32,
    pub(crate) class: Rc<Class>,
    pub(crate) active: bool,
    pub(crate) state: FieldState,
}

impl Entity {
    pub(crate) fn new(index: i32, serial: i32, class: Rc<Class>) -> Self {
        Entity {
            index,
            serial,
            class,
            active: true,
            state: FieldState::new(16),
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }
    pub fn serial(&self) -> i32 {
        self.serial
    }
    pub fn handle(&self) -> i32 {
        self.serial << 14 | self.index
    }
    pub fn class(&self) -> &Class {
        &self.class
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_property_by_name(&self, name: &str) -> Result<&FieldValue> {
        self.get_property_by_field_path(&self.class.get_field_path_for_name(name)?)
    }

    pub(crate) fn get_property_by_field_path(&self, fp: &FieldPath) -> Result<&FieldValue> {
        self.state
            .get_value(fp)
            .ok_or_else(|| anyhow!("No property for given field path"))
    }
}

// ChatGPT -> refactor!!!!!
impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Function to generate a horizontal line
        fn horizontal_line(width1: usize, width2: usize, width3: usize) -> String {
            format!(
                "+{:<width1$}+{:<width2$}+{:<width3$}+\n",
                "-".repeat(width1 + 2),
                "-".repeat(width2 + 2),
                "-".repeat(width3 + 2),
                width1 = width1 + 2,
                width2 = width2 + 2,
                width3 = width3 + 2
            )
        }

        // Function to format a table row
        fn format_row(
            name: &str,
            t: &str,
            value: &str,
            width1: usize,
            width2: usize,
            width3: usize,
        ) -> String {
            format!(
                "| {:<width1$} | {:<width2$} | {:<width3$} |\n",
                name,
                t,
                value,
                width1 = width1,
                width2 = width2,
                width3 = width3
            )
        }

        let mut table = String::new();

        // Calculate column widths based on the longest field name
        let mut name_width = 5; // Minimum width for the "Field" column
        for fp in self
            .class
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let name = self.class.get_name_for_field_path(&fp);
            name_width = name.len().max(name_width);
        }

        let type_width = 35; // Fixed width for the "Type" column
        let value_width = 35; // Fixed width for the "Value" column

        // Add header row
        table += &horizontal_line(name_width, type_width, value_width);
        table += &format_row(
            "Field",
            "Type",
            "Value",
            name_width,
            type_width,
            value_width,
        );
        table += &horizontal_line(name_width, type_width, value_width);

        // Add rows for each field
        for fp in self
            .class
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let t = self.class.get_type_for_field_path(&fp).base.clone();
            let name = self.class.get_name_for_field_path(&fp);
            let value = match self.state.get_value(&fp) {
                Some(v) => match t.as_ref() {
                    "bool" => format!("{}", v.as_bool()),
                    "char" | "CUtlString" | "CUtlSymbolLarge" => format!("\"{}\"", v.as_string()),
                    "int8" | "int16" | "int32" | "int64" => format!("{:?}", v),
                    "uint8"
                    | "uint16"
                    | "uint32"
                    | "uint64"
                    | "CStrongHandle"
                    | "color32"
                    | "CGameSceneNodeHandle"
                    | "Color"
                    | "CUtlStringToken"
                    | "CHandle"
                    | "CEntityHandle"
                    | "CBodyComponent"
                    | "CPhysicsComponent"
                    | "CRenderComponent" => format!("{:?}", v),
                    "float32" | "GameTime_t" | "CNetworkedQuantizedFloat" => {
                        format!("{}", v.as_float())
                    }
                    "Vector2D" => format!(
                        "[{}]",
                        v.as_vector2d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    "Vector3D" | "Vector" | "QAngle" => format!(
                        "[{}]",
                        v.as_vector3d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    "Vector4D" => format!(
                        "[{}]",
                        v.as_vector4d()
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    ),
                    _ => format!("{:?}", v),
                },
                _ => "None".to_string(),
            };
            table += &format_row(&name, &t, &value, name_width, type_width, value_width);
        }

        // Add bottom border
        table += &horizontal_line(name_width, type_width, value_width);

        write!(f, "{}", table)?;
        Ok(())
    }
}
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

    fn try_into(self) -> Result<String, anyhow::Error> {
        match self {
            FieldValue::String(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into String", self)),
        }
    }
}

impl TryInto<String> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<String, anyhow::Error> {
        match self {
            FieldValue::String(x) => Ok(x.to_owned()),
            _ => Err(format_err!("Error converting {} into String", self)),
        }
    }
}

impl TryInto<Box<str>> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Box<str>, anyhow::Error> {
        match self {
            FieldValue::String(x) => Ok(x.into()),
            _ => Err(anyhow!("Error converting {} into Box<str>", self)),
        }
    }
}

impl TryInto<Box<str>> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Box<str>, anyhow::Error> {
        match self {
            FieldValue::String(x) => Ok(x.to_owned().into()),
            _ => Err(anyhow!("Error converting {} into Box<str>", self)),
        }
    }
}

impl TryInto<[f32; 2]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 2], anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 2]", self)),
        }
    }
}

impl TryInto<[f32; 2]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 2], anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 2]", self)),
        }
    }
}

impl TryInto<[f32; 3]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 3], anyhow::Error> {
        match self {
            FieldValue::Vector3D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 3]", self)),
        }
    }
}

impl TryInto<[f32; 3]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 3], anyhow::Error> {
        match self {
            FieldValue::Vector3D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 3]", self)),
        }
    }
}

impl TryInto<[f32; 4]> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 4], anyhow::Error> {
        match self {
            FieldValue::Vector4D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 4]", self)),
        }
    }
}

impl TryInto<[f32; 4]> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 4], anyhow::Error> {
        match self {
            FieldValue::Vector4D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 4]", self)),
        }
    }
}

impl TryInto<Vec<f32>> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<f32>, anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(format_err!("Error converting {} into Vec<f32>", self)),
        }
    }
}

impl TryInto<Vec<f32>> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<f32>, anyhow::Error> {
        match self {
            FieldValue::Vector2D(x) => Ok(x.to_vec()),
            FieldValue::Vector3D(x) => Ok(x.to_vec()),
            FieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(format_err!("Error converting {} into Vec<f32>", self)),
        }
    }
}

impl TryInto<f32> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<f32, anyhow::Error> {
        match self {
            FieldValue::Float(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into f32", self)),
        }
    }
}

impl TryInto<f32> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<f32, anyhow::Error> {
        match self {
            FieldValue::Float(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into f32", self)),
        }
    }
}

impl TryInto<bool> for FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<bool, anyhow::Error> {
        match self {
            FieldValue::Boolean(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into bool", self)),
        }
    }
}

impl TryInto<bool> for &FieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<bool, anyhow::Error> {
        match self {
            FieldValue::Boolean(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into bool", self)),
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
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Float(x) => Ok(x as $target),
                    _ => Err(format_err!(
                        "Cannot convert {} into {}",
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
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Unsigned64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    FieldValue::Float(x) => Ok(*x as $target),
                    _ => Err(format_err!(
                        "Cannot convert {} into {}",
                        self,
                        stringify!($target)
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

impl FieldValue {
    pub fn as_string(&self) -> &str {
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
    //
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
