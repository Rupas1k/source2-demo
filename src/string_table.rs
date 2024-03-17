use crate::reader::Reader;
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
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

    pub fn get_by_id(&self, id: &i32) -> Option<Ref<StringTable>> {
        self.tables.get(id).map(|table| table.borrow())
    }

    pub fn get_by_name(&self, name: &str) -> Option<Ref<StringTable>> {
        self.names_to_table.get(name).map(|table| table.borrow())
    }
}

#[derive(Clone, Debug)]
pub struct StringTableItem {
    pub index: i32,
    pub key: String,
    pub value: Rc<Vec<u8>>,
}

impl StringTableItem {
    pub fn new(index: i32, key: String, value: Rc<Vec<u8>>) -> Self {
        StringTableItem { index, key, value }
    }
}

#[derive(Clone, Debug)]
pub struct StringTable {
    pub index: i32,
    pub name: String,
    pub items: IntMap<i32, StringTableItem>,
    pub user_data_fixed_size: bool,
    pub user_data_size: i32,
    pub flags: u32,
    pub var_int_bit_counts: bool,
}

impl StringTable {
    pub fn parse(&self, buf: &[u8], num_updates: i32) -> Option<Vec<StringTableItem>> {
        let mut items = Vec::<StringTableItem>::new();
        if buf.is_empty() {
            return Some(items);
        }

        let mut r = Reader::new(buf);
        let mut index = -1;
        let mut delta_pos = 0;
        let mut keys = vec![String::new(); 32];

        for _ in 0..num_updates {
            let mut key = String::new();
            let mut value = Vec::<u8>::new();

            index += 1;
            if !r.read_bool() {
                index += r.read_var_u32() as i32 + 1;
            }

            // has key
            if r.read_bool() {
                let delta_zero = if delta_pos > 32 { delta_pos & 31 } else { 0 };
                // use history
                if r.read_bool() {
                    let pos = (delta_zero + r.read_bits(5)) & 31;
                    let size = r.read_bits(5);

                    if delta_pos < pos || keys[pos as usize].len() < size as usize {
                        key = r.read_string().unwrap();
                    } else {
                        key = key
                            + &keys[pos as usize][..(size as usize)]
                            + &r.read_string().unwrap();
                    }
                } else {
                    key = r.read_string().unwrap()
                }
                keys[(delta_pos & 31) as usize] = key.clone();
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
                        r.read_bits(17) * 8
                    }
                };
                value = r.read_bits_as_bytes(bit_size);
                if is_compressed {
                    let mut decoder = snap::raw::Decoder::new();
                    value = decoder.decompress_vec(&value).expect("Error");
                }
            }
            items.push(StringTableItem::new(index, key, Rc::new(value)));
        }
        Some(items)
    }
}
