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
    pub use crate::parser::{Observer, Parser};

    pub use crate::string_table::{StringTable, StringTableEntry};

    pub use crate::entity::{Entity, EntityAction, FieldValue};

    pub use crate::class::Class;

    pub use crate::combat_log::CombatLog;
}

pub use crate::parser::{Observer, Parser};

pub use crate::string_table::{StringTable, StringTableEntry, StringTables};

pub use crate::entity::{Entities, Entity, EntityAction, FieldValue};

pub use crate::class::{Class, Classes};

pub use crate::combat_log::CombatLog;

pub use anyhow::Error;

pub use anyhow::Result;

pub mod proto {
    pub use protogen::prost::Message;

    pub use protogen::*;
}

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
