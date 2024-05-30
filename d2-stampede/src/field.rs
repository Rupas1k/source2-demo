use crate::decoder::Decoder;
use crate::field_value::FieldValue;
use crate::serializer::Serializer;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) struct Field {
    pub(crate) var_name: Box<str>,
    pub(crate) field_type: Rc<FieldType>,
    pub(crate) model: FieldModel,

    pub(crate) decoder: Decoder,
}

impl Field {
    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldVector) -> Vec<FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];
        match &self.model {
            FieldModel::Simple => {
                vec.push(*fp);
            }
            FieldModel::FixedArray | FieldModel::VariableArray(_) => {
                if let Some(s) = st.get_field_vector(fp) {
                    fp.last += 1;
                    for (i, _) in s.state.iter().enumerate() {
                        fp.path[fp.last] = i as u8;
                        vec.push(*fp);
                    }
                    fp.last -= 1;
                }
            }
            FieldModel::FixedTable(serializer) => {
                if let Some(_) = st.get_field_vector(fp) {
                    fp.last += 1;
                    vec.extend(serializer.get_field_paths(fp, st));
                    fp.last -= 1;
                }
            }
            FieldModel::VariableTable(serializer) => {
                if let Some(x) = st.get_field_vector(fp) {
                    fp.last += 2;
                    for (i, v) in x.state.iter().enumerate() {
                        if let StateType::Vector(_) = v {
                            fp.path[fp.last - 1] = i as u8;
                            vec.extend(serializer.get_field_paths(fp, st));
                        }
                    }
                    fp.last -= 2;
                }
            }
        }
        vec
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Encoder {
    Coord,
    SimTime,
    RuneTime,
    Normal,
    Fixed64,
    QAnglePitchYaw,
}

impl Encoder {
    #[inline(always)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "coord" => Some(Encoder::Coord),
            "simtime" => Some(Encoder::SimTime),
            "runetime" => Some(Encoder::RuneTime),
            "normal" => Some(Encoder::Normal),
            "fixed64" => Some(Encoder::Fixed64),
            "qangle_pitch_yaw" => Some(Encoder::QAnglePitchYaw),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct FieldProperties {
    pub encoder: Option<Encoder>,
    pub encoder_flags: i32,
    pub bit_count: i32,
    pub low_value: f32,
    pub high_value: f32,
}

pub enum FieldModel {
    Simple,
    FixedArray,
    VariableArray(Decoder),
    FixedTable(Rc<Serializer>),
    VariableTable(Rc<Serializer>),
}

#[derive(Clone, Debug)]
pub enum StateType {
    Value(FieldValue),
    Vector(FieldVector),
}

impl StateType {
    #[inline(always)]
    pub fn as_field_vector(&self) -> Option<&FieldVector> {
        if let StateType::Vector(x) = self {
            Some(x)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn as_value(&self) -> Option<&FieldValue> {
        if let StateType::Value(x) = self {
            Some(x)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldVector {
    pub(crate) state: Vec<StateType>,
}

impl FieldVector {
    #[inline(always)]
    pub fn new() -> Self {
        FieldVector { state: vec![] }
    }

    #[inline(always)]
    pub fn get_value(&self, fp: &FieldPath) -> Option<&FieldValue> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state
                .state
                .get(fp.path[i] as usize)?
                .as_field_vector()?;
        }
        current_state
            .state
            .get(fp.path[fp.last] as usize)?
            .as_value()
    }

    #[inline(always)]
    pub fn get_field_vector(&self, fp: &FieldPath) -> Option<&FieldVector> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state
                .state
                .get(fp.path[i] as usize)?
                .as_field_vector()?;
        }
        current_state
            .state
            .get(fp.path[fp.last] as usize)?
            .as_field_vector()
    }

    #[inline(always)]
    pub fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        let mut current_state = self;
        for i in 0..=fp.last {
            let index = fp.path[i] as usize;
            if current_state.state.len() <= index {
                current_state
                    .state
                    .resize_with(index + 1, || StateType::Vector(FieldVector::new()))
            }

            if i == fp.last {
                current_state.state[index] = StateType::Value(v);
                return;
            }

            match &mut current_state.state[index] {
                StateType::Value(_) => {
                    current_state.state[index] = StateType::Vector(FieldVector::new());
                }
                StateType::Vector(_) => {}
            }

            match &mut current_state.state[index] {
                StateType::Vector(x) => {
                    current_state = x;
                }
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FieldPath {
    pub(crate) path: [u8; 7],
    pub(crate) last: usize,
}

impl Display for FieldPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..=self.last {
            write!(f, "{}", self.path[i])?;
            if i != self.last {
                write!(f, "/")?;
            }
        }
        Ok(())
    }
}

impl FieldPath {
    #[inline(always)]
    pub(crate) fn new() -> Self {
        FieldPath {
            path: [u8::MAX, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }

    #[inline(always)]
    pub fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            self.last -= 1;
        }
    }

    #[inline(always)]
    pub fn inc(&mut self, n: usize, val: u8) {
        self.path[n] = self.path[n].wrapping_add(val)
    }

    #[inline(always)]
    pub fn sub(&mut self, n: usize, val: u8) {
        self.path[n] = self.path[n].wrapping_sub(val)
    }

    #[inline(always)]
    pub fn inc_curr(&mut self, val: u8) {
        self.path[self.last] = self.path[self.last].wrapping_add(val)
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"([^<\[*]+)(<\s(.*)\s>)?(\*)?(\[(.*)])?").unwrap();
}

#[derive(Clone, Debug)]
pub struct FieldType {
    pub base: Box<str>,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}

impl FieldType {
    pub fn new(name: &str) -> Self {
        let captures = RE.captures(name).unwrap();

        let base = captures[1].to_string().into_boxed_str();
        let pointer = captures.get(4).is_some();
        let generic = captures
            .get(3)
            .map(|v| Box::new(FieldType::new(v.as_str())));

        let count = captures.get(6).map(|x| match x.as_str() {
            "MAX_ITEM_STOCKS" => 8,
            "MAX_ABILITY_DRAFT_ABILITIES" => 48,
            s => s.parse::<i32>().unwrap(),
        });

        FieldType {
            base,
            generic,
            pointer,
            count,
        }
    }

    pub fn as_string(&self) -> String {
        let mut x = self.base.to_string();
        if let Some(generic) = &self.generic {
            x = x + "<" + &generic.as_string() + ">";
        }
        if self.pointer {
            x += "*";
        }
        if let Some(c) = self.count {
            x = x + "[" + &c.to_string() + "]";
        }
        x
    }
}
