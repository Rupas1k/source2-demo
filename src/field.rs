use std::rc::Rc;
use protogen::netmessages::{CSVCMsg_FlattenedSerializer, ProtoFlattenedSerializerField_t};
use crate::field_path::FieldPath;
use crate::field_decoder::Decoders;
use crate::field_state::{FieldState, States};
use crate::field_type::FieldType;
use crate::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct Field {
    pub var_name: Box<str>,
    pub var_type: Box<str>,
    pub serializer_name: Box<str>,
    pub encoder: Box<str>,
    pub encoder_flags: Option<i32>,
    pub bit_count: Option<i32>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub field_type: Option<Rc<FieldType>>,
    pub serializer: Option<Rc<Serializer>>,
    pub value: Option<i32>,
    pub model: FieldModels,

    pub decoder: Option<Decoders>,
    pub base_decoder: Option<Decoders>,
    pub child_decoder: Option<Decoders>,
}

impl Field {
    pub fn new(ser: CSVCMsg_FlattenedSerializer, f: ProtoFlattenedSerializerField_t) -> Self {
        let resolve = |p: Option<i32>| {
            if p.is_none() {
                return "".to_string().into_boxed_str();
            }
            ser.symbols[p.unwrap() as usize].clone().into_boxed_str()
        };

        Field {
            var_name: resolve(f.var_name_sym),
            var_type: resolve(f.var_type_sym),
            serializer_name: resolve(f.field_serializer_name_sym),
            encoder: resolve(f.var_encoder_sym),
            encoder_flags: f.encode_flags,
            bit_count: f.bit_count,
            low_value: f.low_value,
            high_value: f.high_value,
            field_type: None,
            serializer: None,
            value: None,
            model: FieldModels::Simple,

            decoder: Some(Decoders::Default),
            base_decoder: Some(Decoders::Default),
            child_decoder: Some(Decoders::Default),
        }
    }

    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        let mut x = vec![self.var_name.clone().to_string()];
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                if fp.last() == pos as usize {
                    x.push(format!("{:04}", fp.get(pos as usize)));
                }
            }
            FieldModels::FixedTable => {
                if fp.last() >= pos as usize {
                    x.extend_from_slice(&self.serializer.as_ref().unwrap().get_name_for_field_path(fp, pos));
                }
            }
            FieldModels::VariableArray => {
                if fp.last() == pos as usize {
                    x.push(format!("{:04}", fp.get(pos as usize)));
                }
            }
            FieldModels::VariableTable => {
                if fp.last() != (pos - 1) as usize {
                    x.push(format!("{:04}", fp.get(pos as usize)));
                    if fp.last() != pos as usize {
                        x.extend_from_slice(&self.serializer.as_ref().unwrap().get_name_for_field_path(fp, pos + 1))
                    }
                }
            }
        };
        x
    }

    pub fn get_type_for_field_path(&self, fp: &FieldPath, pos: i32) -> &FieldType {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return self.field_type.as_ref().unwrap();
            }
            FieldModels::FixedTable => {
                if fp.last() as i32 != pos - 1 {
                    return self.serializer.as_ref().unwrap().get_type_for_field_path(fp, pos);
                }
            }
            FieldModels::VariableArray => {
                if fp.last() as i32 == pos {
                    return self.field_type.as_ref().unwrap().generic.as_ref().unwrap();
                }
            }
            FieldModels::VariableTable => {
                if fp.last() as i32 >= pos + 1 {
                    return self.serializer.as_ref().unwrap().get_type_for_field_path(fp, pos + 1);
                }
            }
        };
        self.field_type.as_ref().unwrap()
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: i32) -> &Decoders {
        match self.model {
            FieldModels::Simple => {}
            FieldModels::FixedArray => {
                return self.decoder.as_ref().unwrap();
            }
            FieldModels::FixedTable => {
                if fp.last() as i32 == pos - 1 {
                    return self.base_decoder.as_ref().unwrap();
                }
                return self.serializer.as_ref().unwrap().get_decoder_for_field_path(fp, pos);
            }
            FieldModels::VariableArray => {
                if fp.last() as i32 == pos {
                    return self.child_decoder.as_ref().unwrap();
                }
                return self.base_decoder.as_ref().unwrap();
            }
            FieldModels::VariableTable => {
                if fp.last() as i32 >= pos + 1 {
                    return self.serializer.as_ref().unwrap().get_decoder_for_field_path(fp, pos + 1);
                }
                return self.base_decoder.as_ref().unwrap();
            }
        }
        self.decoder.as_ref().unwrap()
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> bool {
        match self.model {
            FieldModels::Simple => {
                panic!("not supported")
            }
            FieldModels::FixedArray => {
                if name.len() != 4 { panic!("wrong size") }
                fp.set(fp.last(), name.parse::<i64>().unwrap());
                return true;
            }
            FieldModels::FixedTable => {
                return self.serializer.as_ref().unwrap().get_field_path_for_name(fp, &name);
            }
            FieldModels::VariableArray => {
                if name.len() != 4 { panic!("wrong size") }
                fp.set(fp.last(), name.parse::<i64>().unwrap());
            }
            FieldModels::VariableTable => {
                if name.len() != 6 { panic!("wrong size") }
                fp.set(fp.last(), name[0..4].parse::<i64>().unwrap());
                fp.down();
                return self.serializer.as_ref().unwrap().get_field_path_for_name(fp, &name[5..]);
            }
        }
        false
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];
        match self.model {
            FieldModels::Simple => {
                vec.push(fp.clone());
            }
            FieldModels::FixedArray => {
                if let Some(States::FieldState(s)) = st.get(fp) {
                    fp.down();
                    for (i, v) in s.state.iter().enumerate() {
                        if v.is_some() {
                            fp.set(fp.last(), i as i64);
                            vec.push(fp.clone());
                        }
                    }
                    fp.up(1);
                }
            }
            FieldModels::FixedTable => {
                if let Some(States::FieldState(v)) = st.get(fp) {
                    fp.down();
                    vec.extend_from_slice(&self.serializer.as_ref().unwrap().get_field_paths(fp, &v));
                    fp.up(1);
                }
            }
            FieldModels::VariableArray => {
                if let Some(States::FieldState(x)) = st.get(fp) {
                    fp.down();
                    for (i, v) in x.state.iter().enumerate() {
                        fp.set(fp.last(), i as i64);
                        vec.push(fp.clone());
                    }
                    fp.up(1);
                }
            }
            FieldModels::VariableTable => {
                if let Some(States::FieldState(x)) = st.get(fp) {
                    fp.down();
                    fp.down();
                    for (i, v) in x.state.iter().enumerate() {
                        if let Some(States::FieldState(vv)) = v.as_ref() {
                            fp.set(fp.last() - 1, i as i64);
                            vec.extend_from_slice(&self.serializer.as_ref().unwrap().get_field_paths(fp, &vv));
                        }
                    }
                    fp.up(2);
                }
            }
        }
        vec
    }

    pub fn set_model(&mut self, model: FieldModels) {
        self.model = model;
        match self.model {
            FieldModels::FixedArray => {
                self.decoder = Some(Decoders::from_field(self, false));
            }
            FieldModels::FixedTable => {
                self.base_decoder = Some(Decoders::Boolean)
            }
            FieldModels::VariableArray => {
                if self.field_type.as_ref().unwrap().generic.is_none() {
                    panic!("No generic")
                }
                self.base_decoder = Some(Decoders::Unsigned);
                self.child_decoder = Some(Decoders::from_field(self, true))
            }
            FieldModels::VariableTable => {
                self.base_decoder = Some(Decoders::Unsigned);
            }
            FieldModels::Simple => {
                self.decoder = Some(Decoders::from_field(self, false));
            }
        }
    }

    pub fn get_name(&self) -> &str {
        &self.var_name
    }
}

#[derive(Clone, Debug)]
pub enum FieldModels {
    Simple,
    FixedArray,
    FixedTable,
    VariableArray,
    VariableTable
}

impl FieldModels {
    pub fn as_str(&self) -> &str {
        match &self {
            FieldModels::Simple => "fixed-array",
            FieldModels::FixedArray => "fixed-table",
            FieldModels::FixedTable => "variable-array",
            FieldModels::VariableArray => "variable-table",
            FieldModels::VariableTable => "simple"
        }
    }
}
