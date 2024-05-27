use crate::serializer::Serializer;
use anyhow::{anyhow, Context, Result};
use prettytable::{row, Table};
use rustc_hash::FxHashMap;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct Classes {
    pub(crate) classes_vec: Vec<Rc<Class>>,
    pub(crate) classes_by_name: FxHashMap<Box<str>, Rc<Class>>,
    pub(crate) class_id_size: Option<u32>,
}

impl Classes {
    pub(crate) fn new() -> Self {
        Classes {
            classes_vec: vec![],
            classes_by_name: FxHashMap::default(),
            class_id_size: None,
        }
    }

    pub(crate) fn get_by_id_rc(&self, id: usize) -> Result<&Rc<Class>> {
        Ok(&self.classes_vec[id])
        // .ok_or_else(|| anyhow!("No class for given id {}", id))
    }

    pub fn get_by_id(&self, id: usize) -> Result<&Class> {
        self.classes_vec
            .get(id)
            .with_context(|| anyhow!("No class for given id {}", id))
            .map(|class| class.as_ref())
    }

    pub fn get_by_name(&self, name: &str) -> Result<&Class> {
        self.classes_by_name
            .get(name)
            .with_context(|| anyhow!("No class for given name \"{}\"", name))
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

impl Display for Classes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["id", "name"]);
        for class in self.classes_vec.iter() {
            table.add_row(row![class.id().to_string(), class.name]);
        }
        write!(f, "{}", table)
    }
}
