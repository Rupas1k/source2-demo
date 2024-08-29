use crate::entity::field::*;
use crate::entity::*;
use crate::event::*;
use crate::string_table::*;
use hashbrown::HashMap;
use std::rc::Rc;

/// Current replay state.
pub struct Context {
    pub(crate) classes: Classes,
    pub(crate) entities: Entities,
    pub(crate) string_tables: StringTables,
    pub(crate) game_events: GameEventList,

    pub(crate) tick: u32,
    pub(crate) previous_tick: u32,
    pub(crate) net_tick: u32,

    pub(crate) game_build: u32,

    pub(crate) baselines: BaselineContainer,
    pub(crate) serializers: HashMap<Box<str>, Rc<Serializer>>,
    pub(crate) last_full_packet_tick: u32,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            classes: Classes::default(),
            entities: Entities::default(),
            string_tables: StringTables::default(),
            game_events: Default::default(),
            tick: u32::MAX,
            previous_tick: u32::MAX,
            net_tick: u32::MAX,
            game_build: 0,
            baselines: BaselineContainer::default(),
            serializers: HashMap::default(),
            last_full_packet_tick: u32::MAX,
        }
    }
}

impl Context {
    pub fn classes(&self) -> &Classes {
        &self.classes
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn string_tables(&self) -> &StringTables {
        &self.string_tables
    }

    pub fn game_events(&self) -> &GameEventList {
        &self.game_events
    }

    pub fn tick(&self) -> u32 {
        self.tick
    }

    pub fn net_tick(&self) -> u32 {
        self.net_tick
    }

    pub fn game_build(&self) -> u32 {
        self.game_build
    }
}
