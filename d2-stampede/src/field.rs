use crate::decoder::Decoders;
use crate::field_value::FieldValue;
use crate::serializer::Serializer;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::max;
use std::rc::Rc;

pub struct Field {
    pub var_name: Box<str>,
    pub field_type: Rc<FieldType>,
    pub model: FieldModels,

    pub decoder: Decoders,
}

impl Field {
    pub fn get_field_paths(
        &self,
        fp: &mut FieldPath,
        st: &FieldState,
    ) -> impl Iterator<Item = FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];
        match &self.model {
            FieldModels::Simple => {
                vec.push(*fp);
            }
            FieldModels::FixedArray | FieldModels::VariableArray(_) => {
                if let Some(s) = st.get_field_state(fp) {
                    fp.last += 1;
                    for (i, _) in s.state.iter().enumerate() {
                        fp.path[fp.last] = i as u8;
                        vec.push(*fp);
                    }
                    fp.pop(1);
                }
            }
            FieldModels::FixedTable(serializer) => {
                if let Some(v) = st.get_field_state(fp) {
                    fp.last += 1;
                    vec.extend(serializer.get_field_paths(fp, v));
                    fp.pop(1);
                }
            }
            FieldModels::VariableTable(serializer) => {
                if let Some(x) = st.get_field_state(fp) {
                    fp.last += 2;
                    for (i, v) in x.state.iter().enumerate() {
                        if let Some(StateType::FieldState(vv)) = v.as_ref() {
                            fp.path[fp.last - 1] = i as u8;
                            vec.extend(serializer.get_field_paths(fp, vv));
                        }
                    }
                    fp.pop(2);
                }
            }
        }
        vec.into_iter()
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

pub enum FieldModels {
    Simple,
    FixedArray,
    VariableArray(Decoders),
    FixedTable(Rc<Serializer>),
    VariableTable(Rc<Serializer>),
}

pub struct FieldPatch {
    pub min_build: u32,
    pub max_build: u32,
    pub patch: fn(&mut FieldProperties, &str),
}

impl FieldPatch {
    pub fn should_apply(&self, build: u32) -> bool {
        if self.min_build == 0 && self.max_build == 0 {
            true
        } else {
            build >= self.min_build && build <= self.max_build
        }
    }
}

#[derive(Clone)]
pub enum StateType {
    Value(FieldValue),
    FieldState(FieldState),
}

impl StateType {
    #[inline(always)]
    pub fn as_field_state(&self) -> Option<&FieldState> {
        if let StateType::FieldState(x) = self {
            Some(x)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn as_field_state_mut(&mut self) -> &mut FieldState {
        if let StateType::FieldState(x) = self {
            return x;
        }
        unreachable!()
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

#[derive(Clone)]
pub struct FieldState {
    pub(crate) state: Vec<Option<StateType>>,
}

impl FieldState {
    #[inline(always)]
    pub fn new(len: usize) -> Self {
        FieldState {
            state: vec![None; len],
        }
    }

    #[inline(always)]
    pub fn get_value(&self, fp: &FieldPath) -> Option<&FieldValue> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state.state[fp.path[i] as usize]
                .as_ref()?
                .as_field_state()?;
        }
        current_state.state[fp.path[fp.last] as usize]
            .as_ref()?
            .as_value()
    }

    #[inline(always)]
    pub fn get_field_state(&self, fp: &FieldPath) -> Option<&FieldState> {
        let mut current_state = self;
        for i in 0..fp.last {
            current_state = current_state.state[fp.path[i] as usize]
                .as_ref()?
                .as_field_state()?;
        }
        current_state.state[fp.path[fp.last] as usize]
            .as_ref()?
            .as_field_state()
    }

    #[inline(always)]
    pub fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        unsafe {
            let mut current_state = self;
            for i in 0..=fp.last {
                if current_state.state.len() <= fp.path[i] as usize {
                    current_state.state.resize_with(
                        max(fp.path[i] as usize + 2, current_state.state.len() * 2),
                        || None,
                    );
                }
                if i == fp.last {
                    current_state.state[fp.path[i] as usize] = Some(StateType::Value(v));
                    return;
                }
                if current_state.state[fp.path[i] as usize].is_none()
                    || !matches!(
                        current_state.state[fp.path[i] as usize]
                            .as_ref()
                            .unwrap_unchecked(),
                        StateType::FieldState(_)
                    )
                // current_state.state[fp.path[i] as usize]
                //         .as_ref()
                //         .unwrap_unchecked()
                //         .as_field_state()
                //         .is_none()
                {
                    current_state.state[fp.path[i] as usize] =
                        Some(StateType::FieldState(FieldState::new(16)));
                }
                current_state = current_state.state[fp.path[i] as usize]
                    .as_mut()
                    .unwrap_unchecked()
                    .as_field_state_mut()
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FieldPath {
    pub(crate) path: [u8; 7],
    pub(crate) last: usize,
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

        let count = captures.get(6).and_then(|x| {
            COUNT_TYPES
                .get(x.as_str())
                .copied()
                .or_else(|| x.as_str().parse().ok())
        });

        FieldType {
            base,
            generic,
            pointer,
            count,
        }
    }
}

lazy_static! {
    pub static ref FIELD_PATCHES: [FieldPatch; 1] = [FieldPatch {
        min_build: 0,
        max_build: 0,
        patch: |properties: &mut FieldProperties, var_name: &str| match var_name {
            "m_flSimulationTime" | "m_flAnimTime" => {
                properties.encoder = Some(Encoder::SimTime);
            }
            "m_flRuneTime" => {
                properties.encoder = Some(Encoder::RuneTime);
            }
            _ => {}
        },
    },];
    static ref RE: Regex = Regex::new(r"([^<\[*]+)(<\s(.*)\s>)?(\*)?(\[(.*)])?").unwrap();
    static ref COUNT_TYPES: FxHashMap<&'static str, i32> =
        [("MAX_ITEM_STOCKS", 8), ("MAX_ABILITY_DRAFT_ABILITIES", 48)]
            .iter()
            .copied()
            .collect();
}
