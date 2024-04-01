use crate::field::FieldPath;
use crate::field::FieldState;
use crate::field::FieldType;
use crate::serializer::Serializer;
use anyhow::{anyhow, Result};
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Classes {
    pub(crate) classes_by_id: IntMap<i32, Rc<Class>>,
    pub(crate) classes_by_name: FxHashMap<Box<str>, Rc<Class>>,
    pub(crate) class_id_size: Option<u32>,
}

impl Classes {
    pub(crate) fn new() -> Self {
        Classes {
            classes_by_id: IntMap::default(),
            classes_by_name: FxHashMap::default(),
            class_id_size: None,
        }
    }

    pub(crate) fn get_by_id_rc(&self, id: &i32) -> Result<&Rc<Class>> {
        self.classes_by_id
            .get(id)
            .ok_or(anyhow!("No class for given id"))
    }

    pub fn get_by_id(&self, id: &i32) -> Result<&Class> {
        self.classes_by_id
            .get(id)
            .ok_or(anyhow!("No class for given id"))
            .map(|class| class.as_ref())
    }

    pub fn get_by_name(&self, name: &str) -> Result<&Class> {
        self.classes_by_name
            .get(name)
            .ok_or(anyhow!("No class for given name"))
            .map(|class| class.as_ref())
    }
}

#[derive(Clone)]
pub struct Class {
    pub(crate) id: i32,
    pub(crate) name: Box<str>,
    pub(crate) serializer: Rc<Serializer>,
    pub(crate) fp_cache: RefCell<FxHashMap<Box<str>, FieldPath>>,
}

impl Class {
    pub(crate) fn new(id: i32, name: &str, serializer: Rc<Serializer>) -> Self {
        Class {
            id,
            name: name.into(),
            serializer,
            fp_cache: RefCell::new(FxHashMap::default()),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i32 {
        self.id
    }

    pub(crate) fn get_name_for_field_path(&self, fp: &FieldPath) -> String {
        self.serializer.get_name_for_field_path(fp, 0).join(".")
    }

    pub(crate) fn get_type_for_field_path(&self, fp: &FieldPath) -> &FieldType {
        self.serializer.get_type_for_field_path(fp, 0)
    }

    pub(crate) fn get_field_path_for_name(&self, name: &str) -> Result<FieldPath> {
        if !self.fp_cache.borrow().contains_key(name) {
            let mut fp = FieldPath::new();
            self.serializer.get_field_path_for_name(&mut fp, name)?;
            self.fp_cache.borrow_mut().insert(name.into(), fp);
        }
        Ok(self.fp_cache.borrow_mut()[name])
    }

    pub(crate) fn get_field_paths<'a>(
        &'a self,
        fp: &'a mut FieldPath,
        st: &'a FieldState,
    ) -> impl Iterator<Item = FieldPath> + 'a {
        self.serializer.get_field_paths(fp, st)
    }
}
