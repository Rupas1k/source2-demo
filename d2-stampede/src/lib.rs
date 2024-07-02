mod class;
mod combat_log;
mod decoder;
mod entity;
mod field;
mod field_reader;
mod field_value;
mod parser;
mod reader;
mod serializer;
mod string_table;

/// Shortcut for getting property from [`core::Entity`]. Supports
/// formatting. [`try_property`]
#[macro_export]
macro_rules! property {
    ($ent:expr, $fmt:expr, $($arg:tt)*) => {
        $ent.get_property_by_name(&format!($fmt, $($arg)*))?.try_into()?
    };
    ($ent:expr, $fmt:expr) => {{
        $ent.get_property_by_name(&format!($fmt))?.try_into()?
    }};
}

#[macro_export]
macro_rules! try_property {
    ($ent:expr, $fmt:expr, $($arg:tt)*) => {
        $ent
            .get_property_by_name(&format!($fmt, $($arg)*))
            .ok()
            .and_then(|x| {
                x.try_into().ok()
            })
    };

    ($ent:expr, $fmt:expr) => {{
        $ent
            .get_property_by_name(&format!($fmt))
            .ok()
            .and_then(|x| {
                x.try_into().ok()
            })
    }};
}
pub mod prelude {
    pub use crate::combat_log::CombatLogEntry;
    pub use crate::entity::{Entity, EntityEvents};
    pub use crate::field_value::FieldValue;
    pub use crate::parser::{Context, Observer, Parser};
    pub use crate::{property, try_property};
    pub use d2_stampede_protobufs::prost::Message;
}

pub use crate::class::{Class, Classes};
pub use crate::combat_log::CombatLogEntry;
pub use crate::entity::{Entities, Entity, EntityEvents};
pub use crate::field_value::FieldValue;
pub use crate::parser::{Context, Observer, Parser};
pub use crate::string_table::{StringTable, StringTableEntry, StringTables};

pub mod proto {
    pub use d2_stampede_protobufs::prost::Message;
    pub use d2_stampede_protobufs::*;
}

pub use anyhow::Error;
pub use anyhow::Result;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
