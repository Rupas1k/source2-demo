use crate::serializer::Serializer;
use anyhow::{anyhow, Result};
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
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
}

impl Class {
    pub(crate) fn new(id: i32, name: &str, serializer: Rc<Serializer>) -> Self {
        Class {
            id,
            name: name.into(),
            serializer,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
    pub fn id(&self) -> i32 {
        self.id
    }
}
