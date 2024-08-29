// #![doc = include_str!("../README.md")]

mod display;
mod entity;
pub mod error;
mod event;
mod macros;
mod parser;
mod reader;
mod string_table;

pub mod proto {
    pub use source2_demo_protobufs::prost::Message;
    pub use source2_demo_protobufs::*;
}

pub mod prelude {
    pub use crate::entity::{Entity, EntityEvents};
    pub use crate::event::{EventValue, GameEvent, GameEventList};
    pub use crate::parser::*;
    pub use crate::string_table::*;
    pub use crate::{property, try_property};

    pub use source2_demo_macros::*;

    pub use source2_demo_protobufs::prost::Message;
    pub use source2_demo_protobufs::EBaseGameEvents;
    pub use source2_demo_protobufs::EBaseUserMessages;
    pub use source2_demo_protobufs::EDemoCommands;
    pub use source2_demo_protobufs::NetMessages;
    pub use source2_demo_protobufs::SvcMessages;

    #[cfg(feature = "dota")]
    pub use crate::event::CombatLogEntry;

    #[cfg(feature = "dota")]
    pub use source2_demo_protobufs::EDotaUserMessages;

    #[cfg(feature = "deadlock")]
    pub use crate::proto::CitadelUserMessageIds;
    #[cfg(feature = "deadlock")]
    pub use crate::proto::ECitadelGameEvents;
}

pub use crate::entity::field::FieldValue;
pub use crate::entity::*;
pub use crate::event::*;
pub use crate::parser::*;
pub use crate::string_table::*;
pub use source2_demo_macros::*;

#[cfg(feature = "dota")]
pub use crate::event::CombatLogEntry;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
