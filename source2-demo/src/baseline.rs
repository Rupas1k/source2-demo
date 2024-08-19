use crate::field::FieldState;
use hashbrown::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct Baselines {
    pub baselines: HashMap<i32, Rc<Vec<u8>>>,
    pub states: HashMap<i32, FieldState>,
}

impl Baselines {
    pub fn add_baseline(&mut self, id: i32, baseline: Rc<Vec<u8>>) {
        self.baselines.insert(id, baseline);
    }
}
