use crate::parser::Baselines;
use crate::reader::Reader;
use hashbrown::HashMap;
use prettytable::{row, Table};
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(thiserror::Error, Debug)]
pub enum StringTableError {
    #[error("String table not found for the given id {0}")]
    TableNotFoundById(i32),

    #[error("String table not found for the given name {0}")]
    TableNotFoundByName(String),

    #[error("String table entry not found for the given index {0} ({1})")]
    RowNotFoundByIndex(i32, String),
}

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

#[derive(Clone, Default)]
pub struct StringTableRow {
    pub(crate) index: i32,
    pub(crate) key: String,
    pub(crate) value: Option<Rc<Vec<u8>>>,
}

impl StringTableRow {
    pub(crate) fn new(index: i32, key: String, value: Option<Rc<Vec<u8>>>) -> Self {
        StringTableRow { index, key, value }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn value(&self) -> Option<&[u8]> {
        self.value.as_ref().map(|x| x.as_slice())
    }
}

#[derive(Clone, Default)]
pub struct StringTable {
    pub(crate) index: i32,
    pub(crate) name: String,
    pub(crate) items: Vec<StringTableRow>,
    pub(crate) user_data_fixed_size: bool,
    pub(crate) user_data_size: i32,
    pub(crate) flags: u32,
    pub(crate) var_int_bit_counts: bool,
    pub(crate) keys: RefCell<Vec<String>>,
}

impl StringTable {
    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Iterator over string table rows.
    pub fn iter(&self) -> impl Iterator<Item = &StringTableRow> {
        self.items.iter()
    }

    /// Returns [`StringTableRow`] for given index.
    pub fn get_row_by_index(&self, idx: usize) -> Result<&StringTableRow, StringTableError> {
        self.items
            .get(idx)
            .ok_or(StringTableError::RowNotFoundByIndex(
                idx as i32,
                self.name.clone(),
            ))
    }

    pub(crate) fn parse(
        &mut self,
        baselines: &mut Baselines,
        buf: &[u8],
        num_updates: i32,
    ) -> Result<(), StringTableError> {
        let items = &mut self.items;
        let mut r = Reader::new(buf);
        let mut index = -1;
        let mut delta_pos = 0;
        let mut keys = self.keys.borrow_mut();

        for _ in 0..num_updates {
            r.refill();

            index += 1;
            if !r.read_bool() {
                index += r.read_var_u32() as i32 + 1;
            }

            let key = r.read_bool().then(|| {
                let delta_zero = if delta_pos > 32 { delta_pos & 31 } else { 0 };
                let key = if r.read_bool() {
                    let pos = (delta_zero + r.read_bits_no_refill(5) as usize) & 31;
                    let size = r.read_bits_no_refill(5) as usize;

                    if delta_pos < pos || keys[pos].len() < size {
                        r.read_string()
                    } else {
                        let x = String::new();
                        x + &keys[pos][..size] + &r.read_string()
                    }
                } else {
                    r.read_string()
                };
                keys[delta_pos & 31].clone_from(&key);
                delta_pos += 1;
                key
            });

            let value = r.read_bool().then(|| {
                let mut is_compressed = false;
                let bit_size = if self.user_data_fixed_size {
                    self.user_data_size as u32
                } else {
                    if (self.flags & 0x1) != 0 {
                        is_compressed = r.read_bool();
                    }
                    if self.var_int_bit_counts {
                        r.read_ubit_var() * 8
                    } else {
                        r.read_bits_no_refill(17) * 8
                    }
                };

                let value = Rc::new(if is_compressed {
                    let mut decoder = snap::raw::Decoder::new();
                    decoder
                        .decompress_vec(&r.read_bits_as_bytes(bit_size))
                        .unwrap()
                } else {
                    r.read_bits_as_bytes(bit_size)
                });

                if self.name == "instancebaseline" {
                    baselines.add_baseline(key.as_ref().unwrap().parse().unwrap(), value.clone());
                }

                value
            });

            if let Some(x) = items.get_mut(index as usize) {
                if let Some(k) = key {
                    x.key = k;
                }
                x.value = value;
            } else {
                items.push(StringTableRow::new(index, key.unwrap_or_default(), value));
            }
        }

        Ok(())
    }
}

impl Display for StringTables {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["id", "name", "rows"]);
        for string_table in self.iter() {
            table.add_row(row![
                string_table.index.to_string(),
                string_table.name,
                string_table.items.len()
            ]);
        }
        write!(f, "{}", table)
    }
}

impl Display for StringTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["idx", "key", "value"]);
        for entry in self.items.iter() {
            table.add_row(row![entry.index, entry.key, format!("{:?}", entry.value)]);
        }
        write!(f, "{}", table)
    }
}
