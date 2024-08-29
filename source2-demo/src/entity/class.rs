use crate::entity::field::Serializer;
use crate::error::ClassError;
use hashbrown::HashMap;
use std::rc::Rc;

/// Container for classes.
#[derive(Default)]
pub struct Classes {
    pub(crate) classes_vec: Vec<Rc<Class>>,
    pub(crate) classes_by_name: HashMap<Box<str>, Rc<Class>>,
    pub(crate) class_id_size: u32,
}

impl Classes {
    pub(crate) fn get_by_id_rc(&self, id: usize) -> &Rc<Class> {
        &self.classes_vec[id]
    }

    /// Iterator over all classes.
    pub fn iter(&self) -> impl Iterator<Item = &Class> {
        self.classes_vec.iter().map(|class| class.as_ref())
    }

    /// Returns [`Class`] for given id
    pub fn get_by_id(&self, id: usize) -> Result<&Class, ClassError> {
        self.classes_vec
            .get(id)
            .ok_or(ClassError::ClassNotFoundById(id as i32))
            .map(|class| class.as_ref())
    }

    /// Returns [`Class`] for given name
    pub fn get_by_name(&self, name: &str) -> Result<&Class, ClassError> {
        self.classes_by_name
            .get(name)
            .ok_or_else(|| ClassError::ClassNotFoundByName(name.to_string()))
            .map(|class| class.as_ref())
    }
}

/// Entity class
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

    /// Returns name of entity class \
    /// `entity.class().name()`
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Returns id of entity class \
    /// `entity.class().id()`
    pub fn id(&self) -> i32 {
        self.id
    }
}
