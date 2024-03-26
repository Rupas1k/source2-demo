use crate::decoder::Decoders;
use crate::entity::FieldValue;
use crate::serializer::Serializer;
use anyhow::bail;
use anyhow::Result;
use enum_as_inner::EnumAsInner;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::max;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Field {
    pub var_name: Box<str>,
    pub var_type: Box<str>,
    pub serializer_name: Box<str>,
    pub encoder: Box<str>,
    pub encoder_flags: i32,
    pub bit_count: i32,
    pub low_value: f32,
    pub high_value: f32,
    pub field_type: Rc<FieldType>,
    pub serializer: Option<Rc<Serializer>>,
    pub model: FieldModels,

    pub decoder: Decoders,
    pub base_decoder: Decoders,
    pub child_decoder: Decoders,
}

impl Field {
    pub fn get_name_for_field_path(&self, fp: &FieldPath, pos: i32) -> Vec<String> {
        let mut x = vec![self.var_name.as_ref().into()];

        match self.model {
            FieldModels::Simple | FieldModels::FixedArray | FieldModels::VariableArray => {
                if fp.last == pos as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                }
            }
            FieldModels::FixedTable | FieldModels::VariableTable => {
                if fp.last >= pos as usize {
                    x.extend_from_slice(
                        &self
                            .serializer
                            .as_ref()
                            .unwrap()
                            .get_name_for_field_path(fp, pos),
                    );
                }
                if self.model == FieldModels::VariableTable && fp.last != (pos - 1) as usize {
                    x.push(format!("{:04}", fp.path[pos as usize]));
                    if fp.last != pos as usize {
                        x.extend_from_slice(
                            &self
                                .serializer
                                .as_ref()
                                .unwrap()
                                .get_name_for_field_path(fp, pos + 1),
                        )
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
                return self.field_type.as_ref();
            }
            FieldModels::FixedTable => {
                if fp.last as i32 != pos - 1 {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_type_for_field_path(fp, pos);
                }
            }
            FieldModels::VariableArray => {
                if fp.last as i32 == pos {
                    return self.field_type.as_ref().generic.as_ref().unwrap();
                }
            }
            FieldModels::VariableTable => {
                if fp.last as i32 > pos {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_type_for_field_path(fp, pos + 1);
                }
            }
        };
        self.field_type.as_ref()
    }

    pub fn get_decoder_for_field_path(&self, fp: &FieldPath, pos: usize) -> &Decoders {
        match self.model {
            FieldModels::Simple => &self.decoder,
            FieldModels::FixedArray => &self.decoder,
            FieldModels::FixedTable => {
                if fp.last == pos - 1 {
                    return &self.base_decoder;
                }
                return self
                    .serializer
                    .as_ref()
                    .unwrap()
                    .get_decoder_for_field_path(fp, pos);
            }
            FieldModels::VariableArray => {
                if fp.last == pos {
                    return &self.child_decoder;
                }
                &self.base_decoder
            }
            FieldModels::VariableTable => {
                if fp.last > pos {
                    return self
                        .serializer
                        .as_ref()
                        .unwrap()
                        .get_decoder_for_field_path(fp, pos + 1);
                }
                &self.base_decoder
            }
        }
    }

    pub fn get_field_path_for_name(&self, fp: &mut FieldPath, name: &str) -> Result<()> {
        match self.model {
            FieldModels::FixedArray | FieldModels::VariableArray => {
                fp.path[fp.last] = name.parse::<i32>()?;
                Ok(())
            }
            FieldModels::FixedTable => self
                .serializer
                .as_ref()
                .unwrap()
                .get_field_path_for_name(fp, name),
            FieldModels::VariableTable => {
                fp.path[fp.last] = name[0..4].parse::<i32>()?;
                fp.last += 1;
                self.serializer
                    .as_ref()
                    .unwrap()
                    .get_field_path_for_name(fp, &name[5..])
            }
            FieldModels::Simple => unreachable!(),
        }
    }

    pub fn get_field_paths(&self, fp: &mut FieldPath, st: &FieldState) -> Vec<FieldPath> {
        let mut vec: Vec<FieldPath> = vec![];
        match self.model {
            FieldModels::Simple => {
                vec.push(fp.clone());
            }
            FieldModels::FixedArray | FieldModels::VariableArray => {
                if let Some(s) = st.get_field_state(fp) {
                    fp.last += 1;
                    for (i, _) in s.state.iter().enumerate() {
                        fp.path[fp.last] = i as i32;
                        vec.push(fp.clone());
                    }
                    fp.pop(1);
                }
            }
            FieldModels::FixedTable => {
                if let Some(v) = st.get_field_state(fp) {
                    fp.last += 1;
                    vec.extend_from_slice(
                        &self.serializer.as_ref().unwrap().get_field_paths(fp, v),
                    );
                    fp.pop(1);
                }
            }
            FieldModels::VariableTable => {
                if let Some(x) = st.get_field_state(fp) {
                    fp.last += 2;
                    for (i, v) in x.state.iter().enumerate() {
                        if let Some(StateType::FieldState(vv)) = v.as_ref() {
                            fp.path[fp.last - 1] = i as i32;
                            vec.extend_from_slice(
                                &self.serializer.as_ref().unwrap().get_field_paths(fp, vv),
                            );
                        }
                    }
                    fp.pop(2);
                }
            }
        }
        vec
    }

    pub fn get_name(&self) -> &str {
        &self.var_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldModels {
    Simple,
    FixedArray,
    FixedTable,
    VariableArray,
    VariableTable,
}

pub struct FieldPatch {
    min_build: u32,
    max_build: u32,
    pub patch: fn(&mut Field),
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

#[derive(Debug, Clone, EnumAsInner)]
pub enum StateType {
    Value(FieldValue),
    FieldState(FieldState),
}

#[derive(Debug, Clone)]
pub struct FieldState {
    pub(crate) state: Vec<Option<StateType>>,
}

impl FieldState {
    pub fn new(len: usize) -> Self {
        FieldState {
            state: vec![None; len],
        }
    }

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

    // REFACTOR!!!!!!
    pub fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        unsafe {
            let mut current_state = self;
            for i in 0..=fp.last {
                if (current_state.state.len() as i32) <= fp.path[i] {
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
                    || !current_state.state[fp.path[i] as usize]
                        .as_ref()
                        .unwrap_unchecked()
                        .is_field_state()
                {
                    current_state.state[fp.path[i] as usize] =
                        Some(StateType::FieldState(FieldState::new(16)));
                }
                current_state = current_state.state[fp.path[i] as usize]
                    .as_mut()
                    .unwrap_unchecked()
                    .as_field_state_mut()
                    .unwrap_unchecked();
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldPath {
    pub(crate) path: [i32; 7],
    pub(crate) last: usize,
}

impl FieldPath {
    pub(crate) fn new() -> Self {
        FieldPath {
            path: [-1, 0, 0, 0, 0, 0, 0],
            last: 0,
        }
    }
    pub fn pop(&mut self, n: usize) {
        for _ in 0..n {
            self.path[self.last] = 0;
            self.last -= 1;
        }
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
        patch: |f: &mut Field| match f.var_name.as_ref() {
            "m_flSimulationTime" | "m_flAnimTime" => {
                f.encoder = "simtime".into();
            }
            "m_flRuneTime" => {
                f.encoder = "runetime".into();
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
