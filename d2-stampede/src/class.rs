use crate::serializer::Serializer;
use hashbrown::HashMap;
use prettytable::{row, Table};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

/// [`Class`] container decoded from [`CDemoClassInfo`]
///
/// [`CDemoClassInfo`]: crate::proto::CDemoClassInfo
#[derive(Default)]
pub struct Classes {
    pub(crate) classes_vec: Vec<Rc<Class>>,
    pub(crate) classes_by_name: HashMap<Box<str>, Rc<Class>>,
    pub(crate) class_id_size: u32,
}

#[derive(thiserror::Error, Debug)]
pub enum ClassError {
    #[error("Class not found for the given id {0}")]
    ClassNotFoundById(i32),

    #[error("Class not found for the given name {0}")]
    ClassNotFoundByName(String),
}

impl Classes {
    pub(crate) fn get_by_id_rc(&self, id: usize) -> &Rc<Class> {
        &self.classes_vec[id]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Class> {
        self.classes_vec.iter().map(|class| class.as_ref())
    }

    pub fn get_by_id(&self, id: usize) -> Result<&Class, ClassError> {
        self.classes_vec
            .get(id)
            .ok_or(ClassError::ClassNotFoundById(id as i32))
            .map(|class| class.as_ref())
    }

    pub fn get_by_name(&self, name: &str) -> Result<&Class, ClassError> {
        self.classes_by_name
            .get(name)
            .ok_or_else(|| ClassError::ClassNotFoundByName(name.to_string()))
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
    pub(crate) fn new(id: i32, name: Box<str>, serializer: Rc<Serializer>) -> Self {
        Class {
            id,
            name,
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
