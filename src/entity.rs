use crate::class::Class;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_state::States;
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

    pub fn get_by_index(&self, index: &i32) -> Option<&Entity> {
        self.index_to_entity.get(&index)
    }
    pub fn get_by_handle(&self, handle: &i32) -> Option<&Entity> {
        self.get_by_index(&Entities::index_for_handle(handle))
    }
    pub fn get_by_class_id(&self, id: &i32) -> Option<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| &entity.class.borrow().id == id)
    }
    pub fn get_by_class_name(&self, name: &str) -> Option<&Entity> {
        self.index_to_entity
            .values()
            .find(|&entity| entity.class.borrow().name.as_ref() == name)
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
    fp_no_op_cache: RefCell<FxHashSet<String>>,
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

    pub fn get_property_by_name(&self, name: &str) -> Option<&EntityFieldType> {
        if self.fp_no_op_cache.borrow().contains(name) {
            return None;
        }
        if let Some(fp) = self.fp_cache.borrow().get(name) {
            return self.get_property_by_field_path(fp);
        }

        let mut fp = FieldPath::new();
        if self.class.borrow().get_field_path_for_name(&mut fp, name) {
            self.fp_cache.borrow_mut().set(name, fp.clone());
            return self.get_property_by_field_path(&fp);
        } else {
            self.fp_no_op_cache.borrow_mut().insert(name.to_string());
        }
        None
    }

    pub fn get_property_by_field_path(&self, fp: &FieldPath) -> Option<&EntityFieldType> {
        self.state.get(fp).unwrap().as_value()
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
pub enum EntityFieldType {
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

macro_rules! impl_try_into_for_fields {
    ($target:ty) => {
        impl TryInto<$target> for EntityFieldType {
            type Error = ();

            fn try_into(self) -> Result<$target, Self::Error> {
                match self {
                    // EntityFieldType::Boolean(x) => Ok(x == 1 as $target),
                    EntityFieldType::Signed8(x) => Ok(x as $target),
                    EntityFieldType::Signed16(x) => Ok(x as $target),
                    EntityFieldType::Signed32(x) => Ok(x as $target),
                    EntityFieldType::Signed64(x) => Ok(x as $target),
                    EntityFieldType::Unsigned8(x) => Ok(x as $target),
                    EntityFieldType::Unsigned16(x) => Ok(x as $target),
                    EntityFieldType::Unsigned32(x) => Ok(x as $target),
                    EntityFieldType::Unsigned64(x) => Ok(x as $target),
                    EntityFieldType::Float(x) => Ok(x as $target),
                    _ => Err(()),
                }
            }
        }

        impl TryInto<$target> for &EntityFieldType {
            type Error = ();

            fn try_into(self) -> Result<$target, Self::Error> {
                // self.to_owned().try_into()
                match self {
                    // EntityFieldType::Boolean(x) => Ok(x == 1 as $target),
                    EntityFieldType::Signed8(x) => Ok(*x as $target),
                    EntityFieldType::Signed16(x) => Ok(*x as $target),
                    EntityFieldType::Signed32(x) => Ok(*x as $target),
                    EntityFieldType::Signed64(x) => Ok(*x as $target),
                    EntityFieldType::Unsigned8(x) => Ok(*x as $target),
                    EntityFieldType::Unsigned16(x) => Ok(*x as $target),
                    EntityFieldType::Unsigned32(x) => Ok(*x as $target),
                    EntityFieldType::Unsigned64(x) => Ok(*x as $target),
                    EntityFieldType::Float(x) => Ok(*x as $target),
                    _ => Err(()),
                }
            }
        }
    };
}

impl_try_into_for_fields!(i8);
impl_try_into_for_fields!(i16);
impl_try_into_for_fields!(i32);
impl_try_into_for_fields!(i64);
impl_try_into_for_fields!(i128);
impl_try_into_for_fields!(u8);
impl_try_into_for_fields!(u16);
impl_try_into_for_fields!(u32);
impl_try_into_for_fields!(u64);
impl_try_into_for_fields!(u128);
impl_try_into_for_fields!(f32);
impl_try_into_for_fields!(f64);

impl EntityFieldType {
    pub fn as_string(&self) -> &str {
        if let EntityFieldType::String(s) = self {
            s.as_str()
        } else {
            panic!("Tried to read as String, Found {:?}", self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let EntityFieldType::Boolean(b) = self {
            *b
        } else {
            panic!("Tried to read as Boolean, Found {:?}", self);
        }
    }
    //
    pub fn as_float(&self) -> f32 {
        if let EntityFieldType::Float(f) = self {
            *f
        } else {
            panic!("Tried to read as Float, Found {:?}", self);
        }
    }

    pub fn as_vector2d(&self) -> &[f32; 2] {
        if let EntityFieldType::Vector2D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector2D, Found {:?}", self);
        }
    }

    pub fn as_vector3d(&self) -> &[f32; 3] {
        if let EntityFieldType::Vector3D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector3D, Found {:?}", self);
        }
    }

    pub fn as_vector4d(&self) -> &[f32; 4] {
        if let EntityFieldType::Vector4D(v) = self {
            v
        } else {
            panic!("Tried to read as Vector4D, Found {:?}", self);
        }
    }
}
