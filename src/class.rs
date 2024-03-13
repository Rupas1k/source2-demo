use crate::field_decoder::Decoders;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_type::FieldType;
use crate::serializer::Serializer;
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Classes {
    pub(crate) class_base_lines: IntMap<i32, Rc<Vec<u8>>>,
    pub(crate) classes_by_id: IntMap<i32, Rc<RefCell<Class>>>,
    pub(crate) classes_by_name: FxHashMap<Box<str>, Rc<RefCell<Class>>>,
    pub(crate) class_id_size: Option<u32>,
    pub(crate) class_info: bool,
}

// impl Default for Classes {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Classes {
    pub(crate) fn new() -> Self {
        Classes {
            class_base_lines: IntMap::default(),
            classes_by_id: IntMap::default(),
            classes_by_name: FxHashMap::default(),
            class_id_size: None,
            class_info: false,
        }
    }

    pub(crate) fn get_by_id_mut(&self, id: i32) -> Option<&Rc<RefCell<Class>>> {
        self.classes_by_id.get(&id)
    }

    pub(crate) fn get_by_name_mut(&self, name: &str) -> Option<&Rc<RefCell<Class>>> {
        self.classes_by_name.get(name)
    }

    pub fn get_by_id(&self, id: i32) -> Option<Ref<Class>> {
        self.classes_by_id.get(&id).map(|class| class.borrow())
    }

    pub fn get_by_name(&self, name: &str) -> Option<Ref<Class>> {
        self.classes_by_name.get(name).map(|class| class.borrow())
    }
}

#[derive(Clone, Debug)]
pub struct Class {
    pub(crate) id: i32,
    pub(crate) name: Box<str>,
    pub(crate) serializer: Rc<Serializer>,
}

impl Class {
    pub fn new(id: i32, name: &str, serializer: Rc<Serializer>) -> Self {
        Class {
            id,
            name: name.to_string().into_boxed_str(),
            serializer,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath) -> String {
        self.serializer.get_name_for_field_path(fp, 0).join(".")
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath) -> &FieldType {
        self.serializer.get_type_for_field_path(fp, 0)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath) -> &Decoders {
        self.serializer.get_decoder_for_field_path(fp, 0)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        self.serializer.get_field_path_for_name(fp, name)
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        self.serializer.get_field_paths(fp, st)
    }
}
