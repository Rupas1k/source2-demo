use crate::error::EntityError;
use crate::Entity;

/// Container for entities.
pub struct Entities {
    pub(crate) entities_vec: Vec<Option<Entity>>,
}

impl Default for Entities {
    fn default() -> Self {
        Entities {
            entities_vec: vec![None; 8192],
        }
    }
}

impl Entities {
    /// Iterator over all entities.
    /// # Examples
    ///
    /// ```
    /// use source2_demo::prelude::*;
    /// use source2_demo::proto::*;
    ///
    /// #[derive(Default)]
    /// struct MyObs;
    ///
    /// impl Observer for MyObs {
    ///     fn on_tick_start(&mut self, ctx: &Context) -> ObserverResult {
    ///         let dire_heroes = ctx
    ///             .entities()
    ///             .iter()
    ///             .filter(|&e| {
    ///                 e.class().name().starts_with("CDOTA_Hero_Unit")
    ///                     && try_property!(e, u32, "m_iTeamNum") == Some(3)
    ///                     && try_property!(e, u32, "m_hReplicatingOtherHeroModel") == Some(u32::MAX)
    ///             })
    ///             .collect::<Vec<_>>();
    ///         Ok(())
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities_vec.iter().flatten()
    }

    /// Returns [`Entity`] for given index.
    pub fn get_by_index(&self, index: usize) -> Result<&Entity, EntityError> {
        self.entities_vec
            .get(index)
            .and_then(|x| x.as_ref())
            .ok_or(EntityError::IndexNotFound(index))
    }

    /// Returns [`Entity`] for given handle.
    pub fn get_by_handle(&self, handle: usize) -> Result<&Entity, EntityError> {
        self.get_by_index(handle & 0x3fff)
            .map_err(|_| EntityError::HandleNotFound(handle))
    }

    /// Returns [`Entity`] for given class id.
    pub fn get_by_class_id(&self, id: i32) -> Result<&Entity, EntityError> {
        self.iter()
            .find(|&entity| entity.class().id() == id)
            .ok_or(EntityError::ClassIdNotFound(id))
    }

    /// Returns [`Entity`] for given class name.
    pub fn get_by_class_name(&self, name: &str) -> Result<&Entity, EntityError> {
        self.iter()
            .find(|&entity| entity.class().name() == name)
            .ok_or(EntityError::ClassNameNotFound(name.to_string()))
    }
}
