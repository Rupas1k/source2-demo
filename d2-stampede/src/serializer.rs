use crate::decoder::Decoders;
use crate::field::{Field, FieldModels, FieldPath, FieldState, FieldType};
use anyhow::{bail, Result};
use rustc_hash::FxHashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct Serializer {
    pub(crate) fields: Vec<Rc<Field>>,
    pub(crate) fp_cache: RefCell<FxHashMap<Box<str>, FieldPath>>,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {
            fields: vec![],
            fp_cache: RefCell::new(FxHashMap::default()),
        }
    }

    pub(crate) fn get_name_for_field_path(&self, fp: &FieldPath) -> String {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        let mut name = String::new();
        loop {
            name += &current_field.var_name;
            match &current_field.model {
                FieldModels::FixedArray | FieldModels::VariableArray(_) => {
                    name += &format!(".{:04}", fp.path[i + 1]);
                    break;
                }
                FieldModels::VariableTable(serializer) => {
                    i += 1;
                    name += &format!(".{:04}.", fp.path[i]);
                    current_serializer = serializer;
                }
                FieldModels::FixedTable(_) | FieldModels::Simple => break,
            }
            i += 1;
            current_field = &current_serializer.fields[fp.path[i] as usize];
        }
        name
    }

    pub(crate) fn get_type_for_field_path(&self, fp: &FieldPath) -> &FieldType {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        loop {
            match &current_field.model {
                FieldModels::Simple | FieldModels::FixedArray => {
                    return current_field.field_type.as_ref()
                }
                FieldModels::FixedTable(serializer) => {
                    if fp.last == i - 1 {
                        return current_field.field_type.as_ref();
                    }
                    current_serializer = serializer;
                }
                FieldModels::VariableArray(_) => {
                    if fp.last == i {
                        return current_field.field_type.as_ref().generic.as_ref().unwrap();
                    }
                    return current_field.field_type.as_ref();
                }
                FieldModels::VariableTable(serializer) => {
                    if i >= fp.last {
                        return current_field.field_type.as_ref();
                    }
                    i += 1;
                    current_serializer = serializer;
                }
            }
            i += 1;
            current_field = &current_serializer.fields[fp.path[i] as usize];
        }
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath) -> &Decoders {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        loop {
            i += 1;
            match &current_field.model {
                FieldModels::Simple | FieldModels::FixedArray => return &current_field.decoder,
                FieldModels::FixedTable(serializer) => {
                    if fp.last + 1 == i {
                        return &current_field.decoder;
                    }
                    current_serializer = serializer;
                }
                FieldModels::VariableArray(child_decoder) => {
                    if fp.last == i {
                        return child_decoder;
                    }
                    return &current_field.decoder;
                }
                FieldModels::VariableTable(serializer) => {
                    if i >= fp.last {
                        return &current_field.decoder;
                    }
                    i += 1;
                    current_serializer = serializer;
                }
            }
            current_field = &current_serializer.fields[fp.path[i] as usize];
        }
    }

    pub(crate) fn get_field_path_for_name(&self, name: &str) -> Result<FieldPath> {
        if !self.fp_cache.borrow().contains_key(name) {
            let mut current_serializer = self;
            let mut fp = FieldPath::new();
            let mut offset = 0;
            'outer: loop {
                for (i, f) in current_serializer.fields.iter().enumerate() {
                    if &name[offset..] == f.var_name.as_ref() {
                        fp.path[fp.last] = i as u8;
                        break 'outer;
                    }
                    if name[offset..].as_bytes().get(f.var_name.len()) == Some(&b"."[0])
                        && &name[offset..(offset + f.var_name.len())] == f.var_name.as_ref()
                    {
                        fp.path[fp.last] = i as u8;
                        fp.last += 1;
                        offset += f.var_name.len() + 1;
                        match &f.model {
                            FieldModels::FixedArray | FieldModels::VariableArray(_) => {
                                fp.path[fp.last] = name[offset..].parse::<u8>()?;
                                break 'outer;
                            }
                            FieldModels::FixedTable(serializer) => {
                                current_serializer = serializer;
                                continue 'outer;
                            }
                            FieldModels::VariableTable(serializer) => {
                                fp.path[fp.last] = name[offset..(offset + 4)].parse::<u8>()?;
                                fp.last += 1;
                                offset += 5;
                                current_serializer = serializer;
                                continue 'outer;
                            }
                            FieldModels::Simple => unreachable!(),
                        }
                    }
                }
                bail!("No field path for given name \"{}\"", name)
            }
            self.fp_cache.borrow_mut().insert(name.into(), fp);
        }
        Ok(self.fp_cache.borrow()[name])
    }

    pub fn get_field_paths<'a>(
        &'a self,
        fp: &'a mut FieldPath,
        st: &'a FieldState,
    ) -> impl Iterator<Item = FieldPath> + 'a {
        self.fields.iter().enumerate().flat_map(|(i, f)| {
            fp.path[fp.last] = i as u8;
            f.get_field_paths(fp, st)
        })
    }
}
