use hashbrown::HashMap;
use std::rc::Rc;

pub(crate) struct GameEventDefinition {
    pub(crate) name: String,
    pub(crate) keys: Vec<Rc<GameEventKey>>,
    pub(crate) name_to_key: HashMap<String, Rc<GameEventKey>>,
}

#[derive(Debug)]
pub(crate) struct GameEventKey {
    pub(crate) id: i32,
    pub(crate) name: String,
}
