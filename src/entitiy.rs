use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use rustc_hash::{FxHashMap};
use crate::class::Class;
use crate::field_path::FieldPath;
use crate::field_state::{FieldState, States};

#[derive(Debug, Clone)]
pub enum EntityOperation {
    None            = 0,
    Created         = 1 << 0,
    Updated         = 1 << 1,
    Deleted         = 1 << 2,
    Entered         = 1 << 3,
    Left            = 1 << 4,

    CreatedEntered  = EntityOperation::Created as isize | EntityOperation::Entered as isize,
    UpdatedEntered  = EntityOperation::Updated as isize | EntityOperation::Entered as isize,
    DeletedLeft     = EntityOperation::Deleted as isize | EntityOperation::Left as isize
}

impl EntityOperation {
    // fn to_string(&self) -> &str {
    //     match self {
    //         EntityOperations::None => "None",
    //         EntityOperations::Created => "Created",
    //         EntityOperations::Updated => "Updated",
    //         EntityOperations::Deleted => "Deleted",
    //         EntityOperations::Entered => "Entered",
    //         EntityOperations::Left => "Left",
    //         EntityOperations::CreatedEntered => "Created+Entered",
    //         EntityOperations::UpdatedEntered => "Updated+Entered",
    //         EntityOperations::DeletedLeft => "Deleted+Left",
    //     }
    // }
}

#[derive(Debug, Clone)]
struct FpCache {
    cache: FxHashMap<&'static str, FieldPath>
}

impl FpCache {
    pub fn new() -> Self{
        FpCache { cache: FxHashMap::<&str, FieldPath>::default() }
    }

    pub fn get(&self, name: &str) -> Option<&FieldPath> {
        self.cache.get(name)
    }

    pub fn set(&mut self, name: &'static str, fp: FieldPath) {
        self.cache.insert(name, fp);
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub index:      i32,
    pub serial:     i32,
    pub class:      Rc<RefCell<Class>>,
    pub active:     bool,
    pub state:      FieldState,
    // pub fp_cache:   HashMap<String, FieldPath>,
    fp_cache: Rc<RefCell<FpCache>>
    // pub fp_no_op:   HashMap<String, bool>
}

impl Entity {
    pub fn new(index: i32, serial: i32, class: Rc<RefCell<Class>>) -> Self {
        Entity {
            index,
            serial,
            class,
            active: true,
            state: FieldState::new(8),
            fp_cache: Rc::new(RefCell::new(FpCache::new())),
            // fp_no_op: HashMap::new(),
        }
    }

    pub fn get_property_by_name(&self, name: &'static str) -> Option<&EntityFieldType> {
        if let Some(fp) = self.fp_cache.borrow().get(name) {
            return self.get_property_by_field_path(fp);
        }

        let mut fp = FieldPath::new();
        if self.class.borrow().get_field_path_for_name(&mut fp, name) {
            self.fp_cache.borrow_mut().set(name, fp.clone());
            return self.get_property_by_field_path(&fp);
        }
        None
    }

    pub fn get_property_by_field_path(&self, fp: &FieldPath) -> Option<&EntityFieldType> {
        self.state.get(fp)
            .as_ref()
            .unwrap()
            .as_value()
    }

    pub fn map(&self) -> HashMap::<String, Option<EntityFieldType>> {
        let mut values = HashMap::<String, Option<EntityFieldType>>::new();
        for fp in self.class.borrow().get_field_paths(&mut FieldPath::new(), &self.state) {
            if let Some(v) = self.state.get(&fp).clone() {
                if let States::Value(vv) = v {
                    values.insert(self.class.borrow().get_name_for_field_path(&fp).to_string(), Some(vv));
                }
            } else {
                values.insert(self.class.borrow().get_name_for_field_path(&fp).to_string(), None);
            }
        }
        values
    }

    // pub fn print_map(&mut self) -> String {
    //     let mut r = String::new();
    //     for fp in self.class.borrow().get_field_paths(&mut FieldPath::new(), &mut self.state) {
    //         let t = self.class.borrow().get_type_for_field_path(&fp).base;
    //         let name = self.class.borrow().get_name_for_field_path(&fp);
    //         let mut x = String::new();
    //         // println!("{:?}", t);
    //         if let Some(v) = self.state.get(&fp).clone() {
    //             if let States::Value(vv) = v {
    //                 x = format!("Some({})", match t.as_str() {
    //                     "bool" => format!("(bool) {}", vv.downcast::<bool>().unwrap().to_string()),
    //                     "char" | "CUtlString" | "CUtlSymbolLarge" => format!("(String) \"{}\"", vv.downcast::<String>().unwrap().to_string()),
    //                     "int8" | "int16" | "int32" | "int64" => format!("(i32) {}", vv.downcast::<i32>().unwrap().to_string()),
    //                     "uint8" | "uint16" | "uint32" | "uint64" | "CStrongHandle" | "color32" | "CGameSceneNodeHandle" | "Color" | "CUtlStringToken" | "CHandle" | "CEntityHandle" | "CBodyComponent" | "CPhysicsComponent" | "CRenderComponent" => format!("(u64) {}", vv.downcast::<u64>().unwrap().to_string()),
    //                     "float32" | "GameTime_t" | "CNetworkedQuantizedFloat" => format!("(f32) {}", vv.downcast::<f32>().unwrap().to_string()),
    //                     "Vector" | "Vector2D" | "Vector4D" | "QAngle" => format!("(Vec<f32>) [{}]", vv.downcast::<Vec<f32>>().unwrap().iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ")),
    //                     _ => format!("(u32) {}", vv.downcast::<u32>().unwrap().to_string())
    //                 })
    //             }
    //         } else {
    //             x = "None".to_string();
    //         }
    //
    //         let s = format!("\"{}\": {}\n", name, x);
    //         r += &s;
    //     }
    //     r
    // }
}



#[derive(Debug, Clone, PartialEq)]
pub enum EntityFieldType {
    Boolean(bool),
    String(String),
    Signed(i64),
    Unsigned(u64),
    Float(f32),
    Vector2D([f32; 2]),
    Vector3D([f32; 3]),
    Vector4D([f32; 4]),

}

impl EntityFieldType {
    pub fn as_string(&self) -> &str {
        if let EntityFieldType::String(s) = self {
            s.as_str()
        } else {
            panic!("Tried to read as String, Found {:?}", self);
        }
    }

    pub fn as_signed(&self) -> i64 {
        if let EntityFieldType::Signed(s) = self {
            *s
        } else {
            panic!("Tried to read as Signed, Found {:?}", self);
        }
    }

    pub fn as_unsigned(&self) -> u64 {
        if let EntityFieldType::Unsigned(u) = self {
            *u
        } else {
            panic!("Tried to read as Unsigned, Found {:?}", self);
        }
    }

    pub fn as_bool(&self) -> bool {
        if let EntityFieldType::Boolean(b) = self {
            *b
        } else {
            panic!("Tried to read as Boolean, Found {:?}", self);
        }
    }

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
