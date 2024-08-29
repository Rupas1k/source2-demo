use crate::entity::field::*;
use crate::error::SerializerError;
use hashbrown::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default)]
pub(crate) struct Serializer {
    pub(crate) fields: Vec<Rc<Field>>,
    pub(crate) fp_cache: RefCell<HashMap<Box<str>, FieldPath>>,
}

impl Serializer {
    #[inline]
    pub(crate) fn get_name_for_field_path(&self, fp: &FieldPath) -> String {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        let mut name = String::new();
        loop {
            name += &current_field.var_name;
            i += 1;
            match &current_field.model {
                FieldModel::FixedArray | FieldModel::VariableArray(_) => {
                    if fp.last == i {
                        name += &format!(".{:04}", fp.path[i]);
                        break;
                    }
                }
                FieldModel::VariableTable(serializer) => {
                    if fp.last + 1 == i {
                        break;
                    }
                    name += &format!(".{:04}.", fp.path[i]);
                    i += 1;
                    current_serializer = serializer;
                }
                FieldModel::FixedTable(serializer) => {
                    if fp.last + 1 == i {
                        break;
                    }
                    name += ".";
                    current_serializer = serializer;
                }
                FieldModel::Simple => break,
            }
            current_field = &current_serializer.fields[fp.path[i] as usize];
        }
        name
    }

    #[inline]
    pub(crate) fn get_type_for_field_path(&self, fp: &FieldPath) -> &FieldType {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        loop {
            i += 1;
            match &current_field.model {
                FieldModel::Simple | FieldModel::FixedArray => {
                    return current_field.field_type.as_ref()
                }
                FieldModel::FixedTable(serializer) => {
                    if fp.last + 1 == i {
                        return current_field.field_type.as_ref();
                    }
                    current_serializer = serializer;
                }
                FieldModel::VariableArray(_) => {
                    if fp.last == i {
                        return current_field.field_type.as_ref().generic.as_ref().unwrap();
                    }
                    return current_field.field_type.as_ref();
                }
                FieldModel::VariableTable(serializer) => {
                    if i >= fp.last {
                        return current_field.field_type.as_ref();
                    }
                    i += 1;
                    current_serializer = serializer;
                }
            }
            current_field = &current_serializer.fields[fp.path[i] as usize];
        }
    }

    #[inline]
    pub(crate) fn get_decoder_for_field_path(&self, fp: &FieldPath) -> &FieldDecoder {
        let mut i = 0;
        let mut current_serializer = self;
        let mut current_field = &current_serializer.fields[fp.path[i] as usize];
        loop {
            i += 1;
            match &current_field.model {
                FieldModel::Simple | FieldModel::FixedArray => return &current_field.decoder,
                FieldModel::FixedTable(serializer) => {
                    if fp.last + 1 == i {
                        return &current_field.decoder;
                    }
                    current_serializer = serializer;
                }
                FieldModel::VariableArray(child_decoder) => {
                    if fp.last == i {
                        return child_decoder;
                    }
                    return &current_field.decoder;
                }
                FieldModel::VariableTable(serializer) => {
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

    #[inline]
    pub(crate) fn get_field_path_for_name(&self, name: &str) -> Result<FieldPath, SerializerError> {
        if !self.fp_cache.borrow().contains_key(name) {
            let mut current_serializer = self;
            let mut fp = FieldPath::default();
            let mut offset = 0;
            'outer: loop {
                for (i, f) in current_serializer.fields.iter().enumerate() {
                    if &name[offset..] == f.var_name.as_ref() {
                        fp.path[fp.last] = i as u16;
                        break 'outer;
                    }
                    if name[offset..].as_bytes().get(f.var_name.len()) == Some(&b"."[0])
                        && &name[offset..(offset + f.var_name.len())] == f.var_name.as_ref()
                    {
                        fp.path[fp.last] = i as u16;
                        fp.last += 1;
                        offset += f.var_name.len() + 1;
                        match &f.model {
                            FieldModel::FixedArray | FieldModel::VariableArray(_) => {
                                fp.path[fp.last] = name[offset..].parse::<u16>().unwrap();
                                break 'outer;
                            }
                            FieldModel::FixedTable(serializer) => {
                                current_serializer = serializer;
                                continue 'outer;
                            }
                            FieldModel::VariableTable(serializer) => {
                                fp.path[fp.last] =
                                    name[offset..(offset + 4)].parse::<u16>().unwrap();
                                fp.last += 1;
                                offset += 5;
                                current_serializer = serializer;
                                continue 'outer;
                            }
                            FieldModel::Simple => unreachable!(),
                        }
                    }
                }
                return Err(SerializerError::NoFieldPath(name.to_string()));
            }
            self.fp_cache.borrow_mut().insert(name.into(), fp);
        }
        Ok(self.fp_cache.borrow()[name])
    }

    pub(crate) fn get_field_paths<'a>(
        &'a self,
        fp: &'a mut FieldPath,
        st: &'a FieldState,
    ) -> Vec<FieldPath> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(i, f)| {
                fp.path[fp.last] = i as u16;
                f.get_field_paths(fp, st)
            })
            .collect::<Vec<_>>()
    }
}
