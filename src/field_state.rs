use crate::entity::FieldValue;
use crate::field_path::FieldPath;
use enum_as_inner::EnumAsInner;
use std::cmp::max;
use std::vec;

#[derive(Debug, Clone, EnumAsInner)]
pub enum States {
    Value(FieldValue),
    FieldState(FieldState),
}

#[derive(Debug, Clone)]
pub struct FieldState {
    pub(crate) state: Vec<Option<States>>,
}

impl FieldState {
    pub fn new(len: usize) -> Self {
        FieldState {
            state: vec![None; len],
        }
    }

    pub fn get(&self, fp: &FieldPath) -> Option<&States> {
        let mut current_state = self;
        for i in 0..fp.last {
            if current_state.state[fp.path[i] as usize].is_none()
                || current_state.state[fp.path[i] as usize]
                    .as_ref()
                    .unwrap()
                    .is_value()
            {
                return None;
            }
            current_state = current_state.state[fp.path[i] as usize]
                .as_ref()
                .unwrap()
                .as_field_state()
                .unwrap();
        }
        current_state.state[fp.path[fp.last] as usize].as_ref()
    }

    pub fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        let mut current_state = self;
        for i in 0..=fp.last {
            if (current_state.state.len() as i32) <= fp.path[i] as i32 {
                current_state.state.resize_with(
                    max(fp.path[i] as usize + 2, current_state.state.len() * 2),
                    || None,
                );
            }
            if i == fp.last {
                current_state.state[fp.path[i] as usize] = Some(States::Value(v));
                return;
            }
            if current_state.state[fp.path[i] as usize].is_none()
                || !current_state.state[fp.path[i] as usize]
                    .as_ref()
                    .unwrap()
                    .is_field_state()
            {
                current_state.state[fp.path[i] as usize] =
                    Some(States::FieldState(FieldState::new(8)));
            }
            current_state = current_state.state[fp.path[i] as usize]
                .as_mut()
                .unwrap()
                .as_field_state_mut()
                .unwrap();
        }
    }
}
