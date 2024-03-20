mod class;
mod combat_log;
mod entity;
mod field;
mod field_decoder;
mod field_path;
mod field_reader;
mod field_state;
mod field_type;
mod huffman_tree;
mod parser;
mod qfloat;
mod reader;
mod serializer;
mod string_table;

pub mod prelude {
    pub use proto::*;

    pub use prost::Message;

    pub use anyhow::Result;

    pub use crate::parser::{Observer, Parser};

    pub use crate::string_table::{StringTable, StringTableItem};

    pub use crate::entity::{Entity, EntityEvent, EntityFieldValue};

    pub use crate::combat_log::CombatLog;
}

pub use proto;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
