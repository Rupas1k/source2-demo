use std::cell::RefCell;
use std::rc::Rc;
use crate::field::{Decoders, FieldPath, FieldState, FieldType};
use crate::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct Class<> {
    pub id:         i32,
    pub name:       String,
    pub serializer: Rc<RefCell<Serializer>>,
}

impl Class {
    pub fn new(id: i32, name: String, serializer: Rc<RefCell<Serializer>>) -> Self {
        Class{
            id,
            name,
            serializer,
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath) -> String{
        self.serializer.borrow().get_name_for_field_path(fp, 0).join(".")
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath) -> FieldType {
        self.serializer.borrow().get_type_for_field_path(fp, 0)
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath) -> Decoders {
        self.serializer.borrow().get_decoder_for_field_path(fp, 0)
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        self.serializer.borrow().get_field_path_for_name(fp, name)
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        self.serializer.borrow().get_field_paths(fp, st)
    }

}