mod parser;
mod field;
mod serializer;
mod huffman_tree;
mod qfloat;
mod class;
mod entitiy;
mod string_table;
mod field_path;
mod reader;
mod field_op;
mod field_patch;
mod field_decoder;
mod field_state;
mod field_reader;
mod field_type;
mod combat_log;


pub mod prelude {
    pub use prost::Message;

    pub use crate::parser::{
        Parser,
        Observer,
    };

    pub use crate::entitiy::{
        Entity,
        EntityEvent,
        EntityFieldType
    };

    pub use crate::combat_log::CombatLog;
}

pub use proto;

#[cfg(not(feature = "default_alloc"))]
use mimalloc::MiMalloc;
#[cfg(not(feature = "default_alloc"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;