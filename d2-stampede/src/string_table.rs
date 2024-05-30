use crate::parser::Baselines;
use crate::reader::Reader;
use anyhow::{anyhow, Context, Result};
use hashbrown::HashMap;
use prettytable::{row, Table};
use std::cell::{Ref, RefCell};
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub struct StringTables {
    pub(crate) tables: Vec<Rc<RefCell<StringTable>>>,
    pub(crate) name_to_table: HashMap<Box<str>, Rc<RefCell<StringTable>>>,
}

impl StringTables {
    pub(crate) fn new() -> Self {
        StringTables {
            tables: vec![],
            name_to_table: HashMap::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Ref<StringTable>> {
        self.tables.iter().map(|table| table.borrow())
    }

    pub fn get_by_id(&self, id: usize) -> Result<Ref<StringTable>> {
        self.tables
            .get(id)
            .with_context(|| anyhow!("No string table for given id"))
            .map(|table| table.borrow())
    }

    pub fn get_by_name(&self, name: &str) -> Result<Ref<StringTable>> {
        self.name_to_table
            .get(name)
            .with_context(|| anyhow!("No string table for given name"))
            .map(|table| table.borrow())
    }
}

#[derive(Clone)]
pub struct StringTableEntry {
    pub(crate) index: i32,
    pub(crate) key: String,
    pub(crate) value: Option<Rc<Vec<u8>>>,
}

impl StringTableEntry {
    pub(crate) fn new(index: i32, key: String, value: Option<Rc<Vec<u8>>>) -> Self {
        StringTableEntry { index, key, value }
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
    pub(crate) items: Vec<StringTableEntry>,
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

    pub fn iter(&self) -> impl Iterator<Item = &StringTableEntry> {
        self.items.iter()
    }

    pub fn get_entry_by_index(&self, idx: usize) -> Result<&StringTableEntry> {
        self.items
            .get(idx)
            .with_context(|| anyhow!("No string table entry for given index {idx}"))
    }

    #[inline]
    pub(crate) fn parse(
        &mut self,
        baselines: &mut Baselines,
        buf: &[u8],
        num_updates: i32,
    ) -> Result<()> {
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
                items.push(StringTableEntry::new(index, key.unwrap(), value));
            }
        }

        Ok(())
    }
}

impl Display for StringTables {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();
        table.add_row(row!["id", "name"]);
        for string_table in self.iter() {
            table.add_row(row![string_table.index.to_string(), string_table.name]);
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
