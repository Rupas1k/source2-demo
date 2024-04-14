mod class;
mod combat_log;
mod decoder;
mod entity;
mod field;
mod field_reader;
mod parser;
mod reader;
mod serializer;
mod string_table;

pub mod prelude {
    pub use crate::parser::{Observer, Parser};

    pub use crate::string_table::{StringTable, StringTableEntry};

    pub use crate::entity::{Entity, EntityAction, FieldValue};

    pub use crate::class::Class;

    pub use crate::combat_log::CombatLog;

    pub use d2_stampede_protobufs::prost::Message;
}

pub use crate::parser::{Observer, Parser};

pub use crate::string_table::{StringTable, StringTableEntry, StringTables};

pub use crate::entity::{Entities, Entity, EntityAction, FieldValue};

pub use crate::class::{Class, Classes};

pub use crate::combat_log::CombatLog;

pub use anyhow::Error;

pub use anyhow::Result;

pub mod proto {
    pub use d2_stampede_protobufs::prost::Message;

    pub use d2_stampede_protobufs::*;
}

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
