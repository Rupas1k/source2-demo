use std::rc::Rc;
use crate::field::{Field};
use crate::field_decoder::Decoders;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_type::FieldType;

#[derive(Clone, Debug)]
pub struct Serializer {
    pub name: String,
    pub ver: i32,
    pub fields: Vec<Rc<Field>>
}

impl Serializer {
    pub fn new (name: String, ver: i32) -> Self {
        Serializer {
            name,
            ver,
            fields: vec![]
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        self.fields[fp.get(pos as usize) as usize].get_name_for_field_path(fp, pos+1)
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> &FieldType {
        self.fields[fp.get(pos as usize) as usize].get_type_for_field_path(fp, pos+1)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> &Decoders {
        let i = fp.get(pos as usize);
        if self.fields.len() <= i as usize {
            panic!("No field");
        }
        self.fields[i as usize].get_decoder_for_field_path(fp, pos+1)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        for (i, f) in self.fields.iter().enumerate() {
            if name == f.var_name {
                fp.set(fp.last(), i as i64);
                return true;
            }
            if name.starts_with(&format!("{}.", f.var_name)) {
                fp.set(fp.last(), i as i64);
                fp.down();
                return f.get_field_path_for_name(fp, name[f.var_name.len() + 1..].to_string());
            }
        }
        false
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut results: Vec<FieldPath> = vec![];
        for (i, f) in self.fields.iter().enumerate() {
            fp.set(fp.last(), i as i64);
            results.extend_from_slice(&f.get_field_paths(fp, st));
        }
        results
    }
}