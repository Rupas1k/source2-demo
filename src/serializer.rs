use crate::decoder::Decoders;
use crate::field::Field;
use crate::field::FieldPath;
use crate::field::FieldState;
use crate::field::FieldType;
use anyhow::{bail, Result};
use std::rc::Rc;

#[derive(Clone)]
pub struct Serializer {
    pub fields: Vec<Rc<Field>>,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer { fields: vec![] }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        self.fields[fp.path[pos as usize] as usize].get_name_for_field_path(fp, pos + 1)
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> &FieldType {
        self.fields[fp.path[pos as usize] as usize].get_type_for_field_path(fp, pos + 1)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: usize) -> &Decoders {
        self.fields[fp.path[pos] as usize].get_decoder_for_field_path(fp, pos + 1)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> Result<()> {
        for (i, f) in self.fields.iter().enumerate() {
            if name == f.var_name.as_ref() {
                fp.path[fp.last] = i as u8;
                return Ok(());
            }
            if name.as_bytes().get(f.var_name.len()) == Some(&".".as_bytes()[0]) {
                fp.path[fp.last] = i as u8;
                fp.last += 1;
                return f.get_field_path_for_name(fp, &name[(f.var_name.len() + 1)..]);
            }
        }
        bail!("No field path for given name")
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
