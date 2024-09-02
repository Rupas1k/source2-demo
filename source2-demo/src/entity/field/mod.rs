mod decoder;
mod encoder;
mod model;
mod path;
mod properties;
mod serializer;
mod state;
mod r#type;
mod value;

pub(crate) use decoder::*;
pub(crate) use encoder::*;
pub(crate) use model::*;
pub(crate) use path::*;
pub(crate) use properties::*;
pub(crate) use r#type::*;
pub(crate) use serializer::*;
pub(crate) use state::*;
pub use value::*;

use std::rc::Rc;

pub(crate) struct Field {
    pub(crate) var_name: Box<str>,
    pub(crate) field_type: Rc<FieldType>,
    pub(crate) model: FieldModel,

    pub(crate) decoder: FieldDecoder,
}

impl Field {
    pub(crate) fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut field_paths: Vec<FieldPath> = vec![];
        match &self.model {
            FieldModel::Simple => {
                field_paths.push(*fp);
            }
            FieldModel::FixedArray | FieldModel::VariableArray(_) => {
                if let Some(s) = st.get_field_state(fp) {
                    fp.last += 1;
                    for i in 0..s.vec.len() {
                        fp.path[fp.last] = i as u16;
                        field_paths.push(*fp);
                    }
                    fp.last -= 1;
                }
            }
            FieldModel::FixedTable(serializer) => {
                if st.get_field_state(fp).is_some() {
                    fp.last += 1;
                    field_paths.extend(serializer.get_field_paths(fp, st));
                    fp.last -= 1;
                }
            }
            FieldModel::VariableTable(serializer) => {
                if let Some(x) = st.get_field_state(fp) {
                    fp.last += 2;
                    for i in 0..x.vec.len() {
                        fp.path[fp.last - 1] = i as u16;
                        field_paths.extend(serializer.get_field_paths(fp, st));
                    }
                    fp.last -= 2;
                }
            }
        }
        field_paths
    }
}
