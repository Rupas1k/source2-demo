use crate::class::Class;
use crate::field::{FieldPath, FieldVector};
use crate::field_value::FieldValue;
use anyhow::{anyhow, Context, Result};
use prettytable::{row, Table};
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityEvents {
    Created = 1 << 0,
    Updated = 1 << 1,
    Deleted = 1 << 2,
    Entered = 1 << 3,
    Left = 1 << 4,
}

pub struct Entities {
    pub(crate) entities_vec: Vec<Option<Entity>>,
}

impl Entities {
    pub(crate) fn new() -> Self {
        Entities {
            entities_vec: vec![],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities_vec.iter().flatten()
    }

    pub fn get_by_index(&self, index: usize) -> Result<&Entity> {
        self.entities_vec
            .get(index)
            .and_then(|x| x.as_ref())
            .with_context(|| anyhow!("No entities for index \"{}\"", index))
    }

    pub fn get_by_handle(&self, handle: usize) -> Result<&Entity> {
        self.get_by_index(handle & 0x3fff)
            .with_context(|| anyhow!("No entities for handle \"{handle}\""))
    }

    pub fn get_by_class_id(&self, id: i32) -> Result<&Entity> {
        self.iter()
            .find(|&entity| entity.class().id() == id)
            .with_context(|| anyhow!("No entities for class with id {id}"))
    }

    pub fn get_by_class_name(&self, name: &str) -> Result<&Entity> {
        self.iter()
            .find(|&entity| entity.class().name() == name)
            .with_context(|| anyhow!("No entities for class with name {name}"))
    }

    pub fn get_all_by_class_id(&self, id: i32) -> impl Iterator<Item = &Entity> {
        self.iter().filter(move |&entity| entity.class().id() == id)
    }

    pub fn get_all_by_class_name<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &Entity> {
        self.iter()
            .filter(move |&entity| entity.class().name() == name)
    }
}

#[derive(Clone)]
pub struct Entity {
    index: u32,
    serial: u32,
    pub(crate) class: Rc<Class>,
    pub(crate) state: FieldVector,
}

impl Entity {
    pub(crate) fn new(index: u32, serial: u32, class: Rc<Class>, state: FieldVector) -> Self {
        Entity {
            index,
            serial,
            class,
            state,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn serial(&self) -> u32 {
        self.serial
    }

    pub fn handle(&self) -> u32 {
        self.serial << 14 | self.index
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn get_property_by_name(&self, name: &str) -> Result<&FieldValue> {
        self.get_property_by_field_path(&self.class.serializer.get_field_path_for_name(name)?)
    }

    pub(crate) fn get_property_by_field_path(&self, fp: &FieldPath) -> Result<&FieldValue> {
        self.state.get_value(fp).with_context(|| {
            anyhow!(
                "No property for given name \"{}\" ({}, {:?})",
                self.class.serializer.get_name_for_field_path(fp),
                self.class().name(),
                fp
            )
        })
    }
}

impl Display for Entities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["idx", "serial", "handle", "class"]);
        for e in self.entities_vec.iter().flatten() {
            table.add_row(row![
                e.index().to_string(),
                e.serial().to_string(),
                e.handle().to_string(),
                e.class().name(),
            ]);
        }
        write!(f, "{}", table)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table.add_row(row!["#", "Field", "Type", "Value"]);

        for fp in self
            .class
            .serializer
            .get_field_paths(&mut FieldPath::new(), &self.state)
        {
            let field_type = self.class.serializer.get_type_for_field_path(&fp);
            let name = self.class.serializer.get_name_for_field_path(&fp);
            let value = self.state.get_value(&fp);
            if let Some(v) = value {
                table.add_row(row![fp, name, field_type.as_string(), format!("{:?}", v)]);
            } else {
                table.add_row(row![fp, name, field_type.as_string(), "None"]);
            }
        }

        write!(f, "{}", table)
    }
}
