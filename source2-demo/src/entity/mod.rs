mod baseline;
mod class;
mod container;

pub(crate) use baseline::*;
pub(crate) mod field;
pub use class::*;
pub use container::*;

use crate::error::EntityError;
use crate::field::{FieldPath, FieldState};
use crate::FieldValue;
use std::rc::Rc;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EntityEvents {
    Created,
    Updated,
    Deleted,
}

impl EntityEvents {
    #[inline]
    pub(crate) fn from_cmd(cmd: u32) -> Self {
        match cmd {
            0 => EntityEvents::Updated,
            2 => EntityEvents::Created,
            3 => EntityEvents::Deleted,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Entity {
    index: u32,
    serial: u32,
    pub(crate) class: Rc<Class>,
    pub(crate) state: FieldState,
}

impl Entity {
    pub(crate) fn new(index: u32, serial: u32, class: Rc<Class>, state: FieldState) -> Self {
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

    /// Returns [`FieldValue`] for given property name. You can also use
    /// [`property!`] and [`try_property!`] macros.
    ///
    /// # Examples
    ///
    /// ```
    /// use source2_demo::prelude::*;
    ///
    /// #[derive(Default)]
    /// struct MyObs;
    ///
    /// impl Observer for MyObs {
    ///     fn on_entity(
    ///         &mut self,
    ///         ctx: &Context,
    ///         event: EntityEvents,
    ///         entity: &Entity,
    ///     ) -> ObserverResult {
    ///         let x: u8 = entity
    ///             .get_property_by_name("CBodyComponent.m_cellX")?
    ///             .try_into()?;
    ///
    ///         let y: u8 = property!(entity, "CBodyComponent.m_cellY");
    ///
    ///         let z = property!(entity, u8, "CBodyComponent.m_cellY");
    ///
    ///         Ok(())
    ///     }
    /// }
    /// ```
    ///
    /// [`property!`]: crate::property
    /// [`try_property!`]: crate::try_property
    pub fn get_property_by_name(&self, name: &str) -> Result<&FieldValue, EntityError> {
        self.get_property_by_field_path(&self.class.serializer.get_field_path_for_name(name)?)
    }

    pub(crate) fn get_property_by_field_path(
        &self,
        fp: &FieldPath,
    ) -> Result<&FieldValue, EntityError> {
        self.state.get_value(fp).ok_or_else(|| {
            EntityError::PropertyNameNotFound(
                self.class.serializer.get_name_for_field_path(fp),
                self.class.name().to_string(),
                format!("{}", fp),
            )
        })
    }
}
