use std::rc::Rc;
use crate::field_decoder::Decoders;
use crate::field_path::FieldPath;
use crate::field_state::FieldState;
use crate::field_type::FieldType;
use crate::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct Class {
    pub id:         i32,
    pub name:       Box<str>,
    pub serializer: Rc<Serializer>,
}

impl Class {
    pub fn new(id: i32, name: String, serializer: Rc<Serializer>) -> Self {
        Class{
            id,
            name: name.into_boxed_str(),
            serializer,
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath) -> String{
        self.serializer.get_name_for_field_path(fp, 0).join(".")
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath) -> &FieldType {
        self.serializer.get_type_for_field_path(fp, 0)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath) -> &Decoders {
        self.serializer.get_decoder_for_field_path(fp, 0)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        self.serializer.get_field_path_for_name(fp, name)
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        self.serializer.get_field_paths(fp, st)
    }

}