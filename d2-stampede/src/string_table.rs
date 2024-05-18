use crate::reader::Reader;
use anyhow::{anyhow, Result};
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct StringTables {
    pub(crate) tables: IntMap<i32, Rc<RefCell<StringTable>>>,
    pub(crate) names_to_table: FxHashMap<Box<str>, Rc<RefCell<StringTable>>>,
    pub(crate) next_index: i32,
}

impl StringTables {
    pub(crate) fn new() -> Self {
        StringTables {
            tables: IntMap::default(),
            names_to_table: FxHashMap::default(),
            next_index: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Ref<StringTable>> {
        self.tables.values().map(|table| table.borrow())
    }

    pub fn get_by_id(&self, id: &i32) -> Result<Ref<StringTable>> {
        self.tables
            .get(id)
            .ok_or_else(|| anyhow!("No string table for given id"))
            .map(|table| table.borrow())
    }

    pub fn get_by_name(&self, name: &str) -> Result<Ref<StringTable>> {
        self.names_to_table
            .get(name)
            .ok_or_else(|| anyhow!("No string table for given name"))
            .map(|table| table.borrow())
    }
}

#[derive(Clone)]
pub struct StringTableEntry {
    pub(crate) index: i32,
    pub(crate) key: String,
    pub(crate) value: Rc<Vec<u8>>,
}

impl StringTableEntry {
    pub(crate) fn new(index: i32, key: String, value: Rc<Vec<u8>>) -> Self {
        StringTableEntry { index, key, value }
    }

    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &[u8] {
        self.value.as_slice()
    }
}

#[derive(Clone)]
pub struct StringTable {
    pub(crate) index: i32,
    pub(crate) name: String,
    pub(crate) items: IntMap<i32, StringTableEntry>,
    pub(crate) user_data_fixed_size: bool,
    pub(crate) user_data_size: i32,
    pub(crate) flags: u32,
    pub(crate) var_int_bit_counts: bool,
}

impl StringTable {
    pub fn index(&self) -> i32 {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn iter(&self) -> impl Iterator<Item = &StringTableEntry> {
        self.items.values()
    }

    pub fn get_entry_by_index(&self, idx: &i32) -> Result<&StringTableEntry> {
        self.items
            .get(idx)
            .ok_or_else(|| anyhow!("No string table entry for given index {idx}"))
    }

    pub(crate) fn parse(&self, buf: &[u8], num_updates: i32) -> Result<Vec<StringTableEntry>> {
        let mut items = Vec::<StringTableEntry>::new();
        if buf.is_empty() {
            return Ok(items);
        }

        let mut r = Reader::new(buf);
        let mut index = -1;
        let mut delta_pos: usize = 0;
        let mut keys = vec![String::new(); 32];

        for _ in 0..num_updates {
            let mut key = String::new();
            let mut value = Vec::<u8>::new();

            r.refill();

            index += 1;
            if !r.read_bool() {
                index += r.read_var_u32() as i32 + 1;
            }

            // has key
            if r.read_bool() {
                let delta_zero = if delta_pos > 32 { delta_pos & 31 } else { 0 };
                // use history
                if r.read_bool() {
                    let pos = (delta_zero + r.read_bits(5) as usize) & 31;
                    let size = r.read_bits(5) as usize;

                    if delta_pos < pos || keys[pos].len() < size {
                        key = r.read_string();
                    } else {
                        key = key + &keys[pos][..size] + &r.read_string();
                    }
                } else {
                    key = r.read_string()
                }
                keys[delta_pos & 31].clone_from(&key);
                delta_pos += 1;
            }

            // has value
            if r.read_bool() {
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
                value = r.read_bits_as_bytes(bit_size);
                if is_compressed {
                    let mut decoder = snap::raw::Decoder::new();
                    value = decoder.decompress_vec(&value)?;
                }
            }
            items.push(StringTableEntry::new(index, key, Rc::new(value)));
        }
        Ok(items)
    }
}
