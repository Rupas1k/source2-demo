use crate::decoder::Decoder;
use crate::field_value::FieldValue;
use crate::serializer::Serializer;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub(crate) struct Field {
    pub(crate) var_name: Box<str>,
    pub(crate) field_type: Rc<FieldType>,
    pub(crate) model: FieldModel,

    pub(crate) decoder: Decoder,
}

impl Field {
    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut field_paths: Vec<FieldPath> = vec![];
        match &self.model {
            FieldModel::Simple => {
                field_paths.push(*fp);
            }
            FieldModel::FixedArray | FieldModel::VariableArray(_) => {
                if let Some(s) = st.get_field_state(fp) {
                    fp.last += 1;
                    for i in 0..s.vec.len() {
                        fp.path[fp.last] = i as u8;
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
                        fp.path[fp.last - 1] = i as u8;
                        field_paths.extend(serializer.get_field_paths(fp, st));
                    }
                    fp.last -= 2;
                }
            }
        }
        field_paths
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Encoder {
    Coord,
    SimTime,
    RuneTime,
    Normal,
    Fixed64,
    QAnglePitchYaw,
}

impl Encoder {
    #[inline]
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
pub(crate) struct FieldProperties {
    pub encoder: Option<Encoder>,
    pub encoder_flags: i32,
    pub bit_count: i32,
    pub low_value: f32,
    pub high_value: f32,
}

pub(crate) enum FieldModel {
    Simple,
    FixedArray,
    VariableArray(Decoder),
    FixedTable(Rc<Serializer>),
    VariableTable(Rc<Serializer>),
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FieldState {
    pub(crate) vec: Vec<FieldState>,
    pub(crate) value: Option<FieldValue>,
}

impl FieldState {
    #[inline]
    pub(crate) fn get_value(&self, fp: &FieldPath) -> Option<&FieldValue> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state.vec.get(fp.path[i] as usize)?
        }
        current_state
            .vec
            .get(fp.path[fp.last] as usize)?
            .value
            .as_ref()
    }

    #[inline]
    pub(crate) fn get_field_state(&self, fp: &FieldPath) -> Option<&FieldState> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state.vec.get(fp.path[i] as usize)?;
        }
        current_state.vec.get(fp.path[fp.last] as usize)
    }

    #[inline]
    pub(crate) fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        let mut current_state = self;
        for i in 0..=fp.last {
            let index = fp.path[i] as usize;
            if current_state.vec.len() <= index {
                current_state
                    .vec
                    .resize_with(index + 1, FieldState::default)
            }
            current_state = &mut current_state.vec[index];
        }
        current_state.value = Some(v);
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FieldPath {
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
    #[inline]
    pub(crate) fn new() -> Self {
        FieldPath {
            path: [u8::MAX, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }

    #[inline]
    pub(crate) fn push(&mut self, val: u8) {
        self.last += 1;
        self.path[self.last] = val;
    }

    #[inline]
    pub(crate) fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            self.last -= 1;
        }
    }

    #[inline]
    pub(crate) fn inc(&mut self, n: usize, val: u8) {
        self.path[n] = self.path[n].wrapping_add(val)
    }

    #[inline]
    pub(crate) fn sub(&mut self, n: usize, val: u8) {
        self.path[n] = self.path[n].wrapping_sub(val)
    }

    #[inline]
    pub(crate) fn inc_curr(&mut self, val: u8) {
        self.path[self.last] = self.path[self.last].wrapping_add(val)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FieldType {
    pub base: Box<str>,
    pub generic: Option<Box<FieldType>>,
    pub pointer: bool,
    pub count: Option<i32>,
}

impl FieldType {
    pub(crate) fn new(name: &str) -> Self {
        let mut base_end = name.len();
        let mut pointer = false;
        let mut count = None;
        let mut generic = None;

        if name.ends_with('*') {
            pointer = true;
            base_end -= 1;
        }

        if let Some(open_bracket_pos) = name.find('[') {
            let close_bracket_pos = name.find(']').unwrap();
            count = match &name[(open_bracket_pos + 1)..close_bracket_pos] {
                "MAX_ITEM_STOCKS" => Some(8),
                "MAX_ABILITY_DRAFT_ABILITIES" => Some(48),
                s => Some(s.parse::<i32>().unwrap()),
            };
            base_end = open_bracket_pos;
        }

        if let Some(open_angle_pos) = name.find('<') {
            let close_angle_pos = name.rfind('>').unwrap();
            generic = Some(Box::new(FieldType::new(
                name[(open_angle_pos + 1)..close_angle_pos].trim(),
            )));
            base_end = open_angle_pos;
        }

        let base = name[..base_end].trim().to_string().into_boxed_str();

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
            x = x + "< " + &generic.as_string() + " >";
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
