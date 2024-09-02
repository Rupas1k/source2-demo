mod container;
mod row;

pub use container::*;
pub use row::*;

use crate::entity::BaselineContainer;
use crate::error::StringTableError;
use crate::reader::{BitsReader, Reader};
use std::cell::RefCell;
use std::rc::Rc;

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
        baselines: &mut BaselineContainer,
        buf: &[u8],
        num_updates: i32,
    ) -> Result<Vec<i32>, StringTableError> {
        let items = &mut self.items;
        let mut reader = Reader::new(buf);
        let mut index = -1;
        let mut delta_pos = 0;
        let mut keys = self.keys.borrow_mut();

        let mut modified = vec![];

        if self.name == "decalprecache" {
            return Ok(modified);
        }

        for _ in 0..num_updates {
            reader.refill();

            index += 1;
            if !reader.read_bool() {
                index += reader.read_var_u32() as i32 + 1;
            }

            let key = reader.read_bool().then(|| {
                let delta_zero = if delta_pos > 32 { delta_pos & 31 } else { 0 };
                let key = if reader.read_bool() {
                    let pos = (delta_zero + reader.read_bits_no_refill(5) as usize) & 31;
                    let size = reader.read_bits_no_refill(5) as usize;

                    if delta_pos < pos || keys[pos].len() < size {
                        reader.read_string()
                    } else {
                        let x = String::new();
                        x + &keys[pos][..size] + &reader.read_string()
                    }
                } else {
                    reader.read_string()
                };
                keys[delta_pos & 31].clone_from(&key);
                delta_pos += 1;
                key
            });

            let value = reader.read_bool().then(|| {
                let mut is_compressed = false;
                let bit_size = if self.user_data_fixed_size {
                    self.user_data_size as u32
                } else {
                    if (self.flags & 0x1) != 0 {
                        is_compressed = reader.read_bool();
                    }
                    if self.var_int_bit_counts {
                        reader.read_ubit_var() * 8
                    } else {
                        reader.read_bits_no_refill(17) * 8
                    }
                };

                let value = Rc::new(if is_compressed {
                    let mut decoder = snap::raw::Decoder::new();
                    decoder
                        .decompress_vec(&reader.read_bits_as_bytes(bit_size))
                        .unwrap()
                } else {
                    reader.read_bits_as_bytes(bit_size)
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

            modified.push(index);
        }

        Ok(modified)
    }
}
