use crate::event::*;
use hashbrown::HashMap;
use source2_demo_protobufs::CSvcMsgGameEventList;
use std::rc::Rc;

#[derive(Default)]
pub struct GameEventList {
    pub(crate) list: HashMap<i32, Rc<GameEventDefinition>>,
}

impl GameEventList {
    pub(crate) fn new(list: CSvcMsgGameEventList) -> Self {
        let list = list
            .descriptors
            .into_iter()
            .map(|descriptor| {
                let keys = descriptor
                    .keys
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(i, key)| {
                        Rc::new(GameEventKey {
                            id: i as i32,
                            name: key.name().into(),
                        })
                    })
                    .collect::<Vec<_>>();
                let definition = Rc::new(GameEventDefinition {
                    name: descriptor.name().into(),
                    keys: keys.clone(),
                    name_to_key: keys
                        .into_iter()
                        .map(|key| (key.name.clone(), key))
                        .collect(),
                });
                (descriptor.eventid(), definition)
            })
            .collect::<HashMap<_, _>>();

        Self { list }
    }
}
