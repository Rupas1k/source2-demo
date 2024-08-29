use crate::error::StringTableError;
use crate::string_table::*;
use hashbrown::HashMap;

/// String tables container.
#[derive(Default, Clone)]
pub struct StringTables {
    pub(crate) tables: Vec<StringTable>,
    pub(crate) name_to_table: HashMap<String, usize>,
}

impl StringTables {
    /// Iterator over all string tables.
    pub fn iter(&self) -> impl Iterator<Item = &StringTable> {
        self.tables.iter()
    }

    /// Returns [`StringTable`] for given id.
    pub fn get_by_id(&self, id: usize) -> Result<&StringTable, StringTableError> {
        self.tables
            .get(id)
            .ok_or(StringTableError::TableNotFoundById(id as i32))
    }

    /// Returns [`StringTable`] for given name.
    pub fn get_by_name(&self, name: &str) -> Result<&StringTable, StringTableError> {
        self.name_to_table
            .get(name)
            .ok_or_else(|| StringTableError::TableNotFoundByName(name.to_string()))
            .map(|&idx| &self.tables[idx])
    }

    pub(crate) fn get_by_name_mut(
        &mut self,
        name: &str,
    ) -> Result<&mut StringTable, StringTableError> {
        self.name_to_table
            .get(name)
            .ok_or_else(|| StringTableError::TableNotFoundByName(name.to_string()))
            .map(|&idx| self.tables.get_mut(idx).unwrap())
    }
}
