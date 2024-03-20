use crate::class::Class;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_state::States;
use anyhow::{anyhow, bail, format_err, Result};
use nohash_hasher::IntMap;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
use std::fmt::{format, Write};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityEvent {
    Created = 1 << 0,
    Updated = 1 << 1,
    Deleted = 1 << 2,
    Entered = 1 << 3,
    Left = 1 << 4,
}

#[derive(Debug)]
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

    fn serial_for_handle(handle: &i32) -> i32 {
        handle >> 14
    }

    fn index_for_handle(handle: &i32) -> i32 {
        handle & 0x3fff
    }

    fn handle_for_index_and_serial(index: &i32, serial: &i32) -> i32 {
        (serial << 14) | index
    }

    pub fn get_by_index(&self, index: &i32) -> Result<&Entity> {
        self.index_to_entity
            .get(&index)
            .ok_or_else(|| anyhow!(format!("No entities for index {index}")))
    }
    pub fn get_by_handle(&self, handle: &i32) -> Result<&Entity> {
        self.get_by_index(&Entities::index_for_handle(handle))
    }
    pub fn get_by_class_id(&self, id: &i32) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| &entity.class.borrow().id == id)
            .ok_or_else(|| anyhow!(format!("No entities for class with id {id}")))
    }
    pub fn get_by_class_name(&self, name: &str) -> Result<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| entity.class.borrow().name.as_ref() == name)
            .ok_or_else(|| anyhow!(format!("No entities for class with name {name}")))
    }

    pub fn get_all_by_class_id(&self, id: &i32) -> Vec<&Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| &entity.class.borrow().id == id)
            .collect::<Vec<_>>()
    }

    pub fn get_all_by_class_name(&self, name: &str) -> Vec<&Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| entity.class.borrow().name.as_ref() == name)
            .collect::<Vec<_>>()
    }

    pub fn get_all_by_predicate(&self, predicate: &dyn Fn(&Entity) -> bool) -> Vec<&Entity> {
        self.index_to_entity
            .values()
            .filter(|entity| predicate(entity))
            .collect::<Vec<_>>()
    }
}

#[derive(Debug, Clone)]
struct FpCache {
    cache: FxHashMap<String, FieldPath>,
}

impl FpCache {
    pub fn new() -> Self {
        FpCache {
            cache: FxHashMap::<String, FieldPath>::default(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&FieldPath> {
        self.cache.get(name)
    }

    pub fn set(&mut self, name: &str, fp: FieldPath) {
        self.cache.insert(name.to_string(), fp);
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub(crate) index: i32,
    pub(crate) serial: i32,
    pub(crate) class: Rc<RefCell<Class>>,
    pub(crate) active: bool,
    pub(crate) state: FieldState,
    fp_cache: RefCell<FpCache>,
    fp_no_op_cache: RefCell<FxHashSet<Box<str>>>,
}

impl Entity {
    pub(crate) fn new(index: i32, serial: i32, class: Rc<RefCell<Class>>) -> Self {
        Entity {
            index,
            serial,
            class,
            active: true,
            state: FieldState::new(8),
            fp_cache: RefCell::new(FpCache::new()),
            fp_no_op_cache: RefCell::new(FxHashSet::default()),
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
    pub fn class(&self) -> Ref<Class> {
        self.class.borrow()
    }
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_property_by_name(&self, name: &str) -> Result<&EntityFieldValue> {
        if self.fp_no_op_cache.borrow().contains(name) {
            bail!("No op for given property");
        }
        if let Some(fp) = self.fp_cache.borrow().get(name) {
            return self.get_property_by_field_path(fp);
        }

        let mut fp = FieldPath::new();
        if self
            .class
            .borrow()
            .get_field_path_for_name(&mut fp, name)
            .is_ok()
        {
            self.fp_cache.borrow_mut().set(name, fp.clone());
            return self.get_property_by_field_path(&fp);
        } else {
            self.fp_no_op_cache.borrow_mut().insert(name.into());
        }
        bail!("No property for given name")
    }

    pub fn get_property_by_field_path(&self, fp: &FieldPath) -> Result<&EntityFieldValue> {
        if let Some(state) = self.state.get(fp) {
            Ok(state.as_value().unwrap())
        } else {
            bail!("No property for given field path")
        }
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = String::new();

        // Add header row
        table += &format!("+-------------------------------------+-------------------------------------+-------------------------------------+\n");
        table += &format!("| Field                               | Type                                | Value                               |\n");
        table += &format!("+-------------------------------------+-------------------------------------+-------------------------------------+\n");

        // Add rows for each field
        for fp in self
            .class
            .borrow()
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let t = self
                .class
                .borrow()
                .get_type_for_field_path(&fp)
                .base
                .clone();
            let name = self.class.borrow().get_name_for_field_path(&fp);
            let value = match self.state.get(&fp) {
                Some(States::Value(v)) => match t.as_str() {
                    "bool" => format!("(bool) {}", v.as_bool()),
                    "char" | "CUtlString" | "CUtlSymbolLarge" => {
                        format!("(String) \"{}\"", v.as_string())
                    }
                    "int8" | "int16" | "int32" | "int64" => format!("(i64) {:?}", v),
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
                    | "CRenderComponent" => format!("(u64) {:?}", v),
                    "float32" | "GameTime_t" | "CNetworkedQuantizedFloat" => {
                        format!("(f32) {}", v.as_float())
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
                    _ => format!("(u32) {:?}", v),
                },
                _ => "None".to_string(),
            };
            table += &format!("| {:<35} | {:<35} | {:<35} |\n", name, t, value);
        }

        // Add bottom border
        table += &format!("+-------------------------------------+-------------------------------------+-------------------------------------+\n");
        write!(f, "{}", table)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityFieldValue {
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

impl Display for EntityFieldValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityFieldValue::Boolean(val) => write!(f, "{}", val),
            EntityFieldValue::String(val) => write!(f, "{}", val),
            EntityFieldValue::Float(val) => write!(f, "{}", val),
            EntityFieldValue::Vector2D(val) => write!(f, "[{}, {}]", val[0], val[1]),
            EntityFieldValue::Vector3D(val) => write!(f, "[{}, {}, {}]", val[0], val[1], val[2]),
            EntityFieldValue::Vector4D(val) => {
                write!(f, "[{}, {}, {}, {}]", val[0], val[1], val[2], val[3])
            }
            EntityFieldValue::Signed8(val) => write!(f, "{}", val),
            EntityFieldValue::Signed16(val) => write!(f, "{}", val),
            EntityFieldValue::Signed32(val) => write!(f, "{}", val),
            EntityFieldValue::Signed64(val) => write!(f, "{}", val),
            EntityFieldValue::Unsigned8(val) => write!(f, "{}", val),
            EntityFieldValue::Unsigned16(val) => write!(f, "{}", val),
            EntityFieldValue::Unsigned32(val) => write!(f, "{}", val),
            EntityFieldValue::Unsigned64(val) => write!(f, "{}", val),
        }
    }
}

impl TryInto<[f32; 2]> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 2], anyhow::Error> {
        match self {
            EntityFieldValue::Vector2D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 2]", self,)),
        }
    }
}

impl TryInto<[f32; 2]> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 2], anyhow::Error> {
        match self {
            EntityFieldValue::Vector2D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 2]", self,)),
        }
    }
}

impl TryInto<[f32; 3]> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 3], anyhow::Error> {
        match self {
            EntityFieldValue::Vector3D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 3]", self,)),
        }
    }
}

impl TryInto<[f32; 3]> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 3], anyhow::Error> {
        match self {
            EntityFieldValue::Vector3D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 3]", self,)),
        }
    }
}

impl TryInto<[f32; 4]> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 4], anyhow::Error> {
        match self {
            EntityFieldValue::Vector4D(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into [f32; 4]", self,)),
        }
    }
}

impl TryInto<[f32; 4]> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<[f32; 4], anyhow::Error> {
        match self {
            EntityFieldValue::Vector4D(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into [f32; 4]", self,)),
        }
    }
}

impl TryInto<Vec<f32>> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<f32>, anyhow::Error> {
        match self {
            EntityFieldValue::Vector2D(x) => Ok(x.to_vec()),
            EntityFieldValue::Vector3D(x) => Ok(x.to_vec()),
            EntityFieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(format_err!("Error converting {} into Vec<f32>", self,)),
        }
    }
}

impl TryInto<Vec<f32>> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<f32>, anyhow::Error> {
        match self {
            EntityFieldValue::Vector2D(x) => Ok(x.to_vec()),
            EntityFieldValue::Vector3D(x) => Ok(x.to_vec()),
            EntityFieldValue::Vector4D(x) => Ok(x.to_vec()),
            _ => Err(format_err!("Error converting {} into Vec<f32>", self,)),
        }
    }
}

impl TryInto<f32> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<f32, anyhow::Error> {
        match self {
            EntityFieldValue::Float(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into f32", self,)),
        }
    }
}

impl TryInto<f32> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<f32, anyhow::Error> {
        match self {
            EntityFieldValue::Float(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into f32", self,)),
        }
    }
}

impl TryInto<bool> for EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<bool, anyhow::Error> {
        match self {
            EntityFieldValue::Boolean(x) => Ok(x),
            _ => Err(format_err!("Error converting {} into bool", self,)),
        }
    }
}

impl TryInto<bool> for &EntityFieldValue {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<bool, anyhow::Error> {
        match self {
            EntityFieldValue::Boolean(x) => Ok(*x),
            _ => Err(format_err!("Error converting {} into bool", self)),
        }
    }
}

macro_rules! impl_try_into_for_integers {
    ($target:ty) => {
        impl TryInto<$target> for EntityFieldValue {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<$target, anyhow::Error> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok((x == 1) as $target),
                    EntityFieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Unsigned8(x) => {
                        Ok(TryInto::<$target>::try_into(x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Unsigned16(x) => Ok(TryInto::<$target>::try_into(x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Unsigned32(x) => Ok(TryInto::<$target>::try_into(x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Unsigned64(x) => Ok(TryInto::<$target>::try_into(x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Float(x) => Ok(x as $target),
                    _ => Err(format_err!(
                        "Cannot convert {} into {}",
                        self,
                        stringify!($target)
                    )),
                }
            }
        }

        impl TryInto<$target> for &EntityFieldValue {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<$target, anyhow::Error> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok(x == 1 as $target),
                    EntityFieldValue::Signed8(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed16(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed32(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Signed64(x) => {
                        Ok(TryInto::<$target>::try_into(*x).map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?)
                    }
                    EntityFieldValue::Unsigned8(x) => Ok(TryInto::<$target>::try_into(*x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Unsigned16(x) => Ok(TryInto::<$target>::try_into(*x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Unsigned32(x) => Ok(TryInto::<$target>::try_into(*x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Unsigned64(x) => Ok(TryInto::<$target>::try_into(*x)
                        .map_err(|_| {
                            format_err!("Error converting {x} into {}", stringify!($target))
                        })?),
                    EntityFieldValue::Float(x) => Ok(*x as $target),
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

impl EntityFieldValue {
    pub fn as_string(&self) -> &str {
        if let EntityFieldValue::String(s) = self {
            s.as_str()
        } else {
            panic!("Tried to read as String, Found {:?}", self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let EntityFieldValue::Boolean(b) = self {
            *b
        } else {
            panic!("Tried to read as Boolean, Found {:?}", self);
        }
    }
    //
    pub fn as_float(&self) -> f32 {
        if let EntityFieldValue::Float(f) = self {
            *f
        } else {
            panic!("Tried to read as Float, Found {:?}", self);
        }
    }

    pub fn as_vector2d(&self) -> &[f32; 2] {
        if let EntityFieldValue::Vector2D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector2D, Found {:?}", self);
        }
    }

    pub fn as_vector3d(&self) -> &[f32; 3] {
        if let EntityFieldValue::Vector3D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector3D, Found {:?}", self);
        }
    }

    pub fn as_vector4d(&self) -> &[f32; 4] {
        if let EntityFieldValue::Vector4D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector4D, Found {:?}", self);
        }
    }
}
