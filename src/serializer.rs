use crate::field::Field;
use crate::field_decoder::Decoders;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_type::FieldType;
use anyhow::{anyhow, bail, Result};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Serializer {
    pub name: Box<str>,
    pub ver: i32,
    pub fields: Vec<Rc<Field>>,
}

impl Serializer {
    pub fn new(name: String, ver: i32) -> Self {
        Serializer {
            name: name.into_boxed_str(),
            ver,
            fields: vec![],
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        self.fields[fp.path[pos as usize] as usize].get_name_for_field_path(fp, pos + 1)
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> &FieldType {
        self.fields[fp.path[pos as usize] as usize].get_type_for_field_path(fp, pos + 1)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> &Decoders {
        let i = fp.path[pos as usize];
        if self.fields.len() <= i as usize {
            panic!("No field");
        }
        self.fields[i as usize].get_decoder_for_field_path(fp, pos + 1)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> Result<()> {
        for (i, f) in self.fields.iter().enumerate() {
            if name == f.var_name.as_ref() {
                fp.path[fp.last] = i as i32;
                return Ok(());
            }
            if name.starts_with(&format!("{}.", f.var_name)) {
                fp.path[fp.last] = i as i32;
                fp.last += 1;
                return f.get_field_path_for_name(fp, &name[(f.var_name.len() + 1)..]);
            }
        }
        bail!("No field path for given name")
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(i, f)| {
                fp.path[fp.last] = i as i32;
                f.get_field_paths(fp, st)
            })
            .collect()
    }
}
