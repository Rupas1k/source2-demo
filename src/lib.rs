mod class;
mod combat_log;
mod entity;
mod field;
mod field_decoder;
mod field_op;
mod field_patch;
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

// pub mod observers;

pub mod prelude {
    pub use proto::*;

    pub use prost::Message;

    pub use crate::parser::{Observer, Parser};

    pub use crate::entity::{Entity, EntityEvent, EntityFieldType};

    pub use crate::combat_log::CombatLog;
}

// pub use crate::observers;

pub use proto;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
