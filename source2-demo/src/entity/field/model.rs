use crate::entity::field::serializer::Serializer;
use crate::entity::field::FieldDecoder;
use std::rc::Rc;

pub(crate) enum FieldModel {
    Simple,
    FixedArray,
    VariableArray(FieldDecoder),
    FixedTable(Rc<Serializer>),
    VariableTable(Rc<Serializer>),
}
