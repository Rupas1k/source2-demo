use std::collections::{VecDeque};
use std::rc::Rc;
use nohash_hasher::IntMap;
use rustc_hash::FxHashMap;
use crate::reader::{Reader, ReaderMethods};

#[derive(Clone, Debug)]
pub struct StringTables {
    pub tables: IntMap<i32, StringTable>,
    pub name_index: FxHashMap<String, i32>,
    pub next_index: i32,
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

#[derive(Clone, Debug)]
pub struct StringTableItem {
    pub index: i32,
    pub key: String,
    pub value: Rc<Vec<u8>>,
}

impl StringTableItem {
    pub fn new(index: i32, key: String, value: Rc<Vec<u8>>) -> Self {
        StringTableItem {
            index,
            key,
            value,
        }
    }
}

impl StringTable {
    pub fn parse(&self, buf: &[u8], num_updates: i32) -> Option<Vec<StringTableItem>> {
        let mut items = Vec::<StringTableItem>::new();
        if buf.len() == 0 {
            return Some(items);
        }

        let mut r = Reader::new(buf);
        let mut index = -1;
        let mut keys = VecDeque::<String>::new();


        for _ in 0..num_updates {
            let mut key = String::new();
            let mut value = Vec::<u8>::new();
            let incr = r.read_bool();
            if incr {
                index += 1;
            } else {
                index = r.read_var_u32() as i32 + 1;
            }

            let has_key = r.read_bool();

            if has_key {
                let use_history = r.read_bool();
                if use_history {
                    let pos = r.read_bits(5);
                    let size = r.read_bits(5);

                    match r.read_string() {
                        Some(rs) => {
                            if pos as usize >= keys.len() {
                                key += rs.as_str();
                            } else {
                                let s = keys[pos as usize].as_str();
                                if size as usize > s.len() {
                                    key = key + &s + rs.as_str();
                                } else {
                                    key = key + &s[0..(size as usize)] + rs.as_str();
                                }
                            }
                        }
                        None => return None
                    }

                } else {
                    key = match r.read_string() {
                        Some(x) => x,
                        None => return None
                    };
                }

                if keys.len() == 32 {
                    keys.pop_front();
                }
                keys.push_back(key.clone());
            }

            let has_value = r.read_bool();
            if has_value {
                let mut bit_size = 0u32;
                let mut is_compressed = false;
                if self.user_data_fixed_size {
                    bit_size = self.user_data_size as u32;
                } else {
                    if (self.flags & 0x1) != 0 {
                        is_compressed = r.read_bool();
                    }
                    if self.var_int_bit_counts {
                        bit_size = r.read_ubit_var() * 8;
                    } else {
                        bit_size = r.read_bits(17) * 8;
                    }
                }
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

impl StringTables {
    pub fn new() -> Self {
        StringTables {
            tables: IntMap::default(),
            name_index: FxHashMap::default(),
            next_index: 0,
        }
    }

    pub fn get_table_by_name(&self, name: &str) -> Option<&StringTable> {
        match self.name_index.get(name) {
            Some(i) => {
                Some(&self.tables[i])
            }
            None => None
        }
    }
}