use crate::entity::field::{FieldPath, FieldValue};

#[derive(Clone, Debug, Default)]
pub(crate) struct FieldState {
    pub(crate) vec: Vec<FieldState>,
    pub(crate) value: Option<FieldValue>,
}

impl FieldState {
    #[inline]
    pub(crate) fn get_value(&self, fp: &FieldPath) -> Option<&FieldValue> {
        self.get_field_state(fp).and_then(|x| x.value.as_ref())
    }

    #[inline]
    pub(crate) fn get_field_state(&self, fp: &FieldPath) -> Option<&FieldState> {
        let mut current_state = self;
        for i in 0..=fp.last {
            current_state = current_state.vec.get(fp.path[i] as usize)?;
        }
        Some(current_state)
    }

    #[inline]
    pub(crate) fn set(&mut self, fp: &FieldPath, v: FieldValue) {
        let mut current_state = self;
        for i in 0..=fp.last {
            let index = fp.path[i] as usize;
            while current_state.vec.len() <= index {
                current_state.vec.push(FieldState::default());
            }
            current_state = &mut current_state.vec[index];
        }
        current_state.value = Some(v);
    }
}
