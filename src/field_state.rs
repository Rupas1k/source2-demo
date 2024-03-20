use crate::entity::EntityFieldValue;
use crate::field_path::FieldPath;
use enum_as_inner::EnumAsInner;
use std::cmp::max;

#[derive(Debug, Clone, EnumAsInner)]
pub enum States {
    Value(EntityFieldValue),
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
        let mut z = 0;
        for i in 0..=fp.last {
            z = fp.path[i];
            if (current_state.state.len() as i32) < z + 2 {
                return None;
            }
            if i == fp.last {
                return current_state.state[z as usize].as_ref();
            }
            if let States::FieldState(x) = current_state.state[z as usize].as_ref().unwrap() {
                current_state = x;
                continue;
            }
            return None;
        }
        None
    }

    pub fn set(&mut self, fp: &FieldPath, v: EntityFieldValue) {
        let mut x = self;
        for i in 0..=fp.last {
            let z = fp.path[i];
            let y = x.state.len() as i32;
            if y <= z {
                x.state.resize_with(max(z + 2, y * 2) as usize, || None);
            }

            if i == fp.last {
                if x.state[z as usize].as_ref().is_none() {
                    x.state[z as usize] = Some(States::Value(v));
                    return;
                }
                if let States::Value(_) = x.state[z as usize].as_ref().unwrap() {
                    x.state[z as usize] = Some(States::Value(v));
                }
                return;
            }

            if x.state[z as usize].is_none() {
                x.state[z as usize] = Some(States::FieldState(FieldState::new(8)));
            } else if let States::Value(_) = x.state[z as usize].as_ref().unwrap() {
                x.state[z as usize] = Some(States::FieldState(FieldState::new(8)));
            }

            x = x.state[z as usize]
                .as_mut()
                .unwrap()
                .as_field_state_mut()
                .unwrap();
        }
    }
}
