use crate::field::{Decoders, Field, FieldPath, FieldState, FieldType};

#[derive(Clone, Debug)]
pub struct Serializer {
    pub name: String,
    pub ver: i32,
    pub fields: Vec<Field>
}

impl Serializer {
    pub fn new (name: String, ver: i32) -> Self {
        Serializer {
            name,
            ver,
            fields: vec![]
        }
    }

    pub fn id(&self) -> String {
        format!("{}({})", self.name, self.ver)
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        self.fields[fp.path[pos as usize] as usize].get_name_for_field_path(fp, pos+1)
    }

    pub fn get_field_for_field_path(&self, fp: &FieldPath, pos: i32) -> Field {
        self.fields[*fp.path.get(pos as usize).unwrap() as usize].get_field_for_field_path(fp, pos+1).clone()
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> FieldType {
        self.fields[*fp.path.get(pos as usize).unwrap() as usize].get_type_for_field_path(fp, pos+1)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> Decoders {
        let i = fp.path[pos as usize];
        if self.fields.len() <= i as usize {
            panic!("No field");
        }
        self.fields[i as usize].get_decoder_for_field_path(fp, pos+1)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        for (i, f) in self.fields.iter().enumerate() {
            if name == f.var_name {
                fp.path[fp.last] = i as i32;
                return true;
            }
            if name.starts_with(&format!("{}.", f.var_name)) {
                fp.path[fp.last] = i as i32;
                fp.last += 1;
                return f.get_field_path_for_name(fp, name[f.var_name.len() + 1..].to_string());
            }
        }
        false
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut results: Vec<FieldPath> = vec![];
        for (i, f) in self.fields.iter().enumerate() {
            // println!("{:?}", f);
            fp.path[fp.last] = i as i32;
            results.extend_from_slice(&f.get_field_paths(fp, st));
        }
        // println!("{:?}", results.len());
        // panic!();
        results
    }
}