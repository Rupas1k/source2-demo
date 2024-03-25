mod class;
mod combat_log;
mod decoder;
mod entity;
mod field;
mod field_reader;
mod operation;
mod parser;
mod serializer;
mod string_table;
mod utils;

pub mod prelude {
    pub use proto::*;

    pub use prost::Message;

    pub use anyhow::Result;

    pub use crate::parser::{Observer, Parser};

    pub use crate::string_table::{StringTable, StringTableItem};

    pub use crate::entity::{Entity, EntityAction, FieldValue};

    pub use crate::combat_log::CombatLog;
}

pub use proto;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
