#![doc = include_str!("../../README.md")]
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

/// Macro for getting property from [`Entity`].
///
/// # Examples
/// ```no_compile
/// let x: i32 = property!(entity, "property_name");
/// let y = property!(entity, i32, "property_name");
/// ```
#[macro_export]
macro_rules! property {
    ($ent:expr, $ty:ty, $fmt:literal, $($arg:tt)*) => {
        {
            let x: $ty = $ent.get_property_by_name(&format!($fmt, $($arg)*))?.try_into()?;
            x
        }
    };
    ($ent:expr, $ty:ty, $fmt:literal) => {
        {
            let x: $ty = $ent.get_property_by_name(&format!($fmt))?.try_into()?;
            x
        }
    };
    ($ent:expr, $fmt:expr, $($arg:tt)*) => {
        $ent.get_property_by_name(&format!($fmt, $($arg)*))?.try_into()?
    };
    ($ent:expr, $fmt:expr) => {{
        $ent.get_property_by_name(&format!($fmt))?.try_into()?
    }};
}

/// Same as [`property`] but returns `None` if property doesn't exist for given
/// [`Entity`] or cannot be converted into given type.
///
/// # Examples
/// ```no_compile
/// let x: i32 = try_property!(entity, "property_name").unwrap_or_default();
/// let y = try_property!(entity, i32, "property_name").unwrap_or_default();
/// ```
#[macro_export]
macro_rules! try_property {
    ($ent:expr, $ty:ty, $fmt:expr, $($arg:tt)*) => {
        {
            let x: Option<$ty> = $ent
                .get_property_by_name(&format!($fmt, $($arg)*))
                .ok()
                .and_then(|x| {
                    x.try_into().ok()
                });
            x
        }
    };

    ($ent:expr, $ty:ty, $fmt:expr) => {
        {
            let x: Option<$ty> = $ent
                .get_property_by_name(&format!($fmt))
                .ok()
                .and_then(|x| {
                    x.try_into().ok()
                });
            x
        }
    };

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
    pub use crate::ObserverResult;
    pub use crate::{property, try_property};
    pub use d2_stampede_protobufs::prost::Message;
}

pub use crate::class::{Class, Classes};
pub use crate::combat_log::CombatLogEntry;
pub use crate::entity::{Entities, Entity, EntityEvents};
pub use crate::field_value::FieldValue;
pub use crate::parser::{Context, Observer, Parser};
pub use crate::string_table::{StringTable, StringTableRow, StringTables};

pub mod error {
    pub use crate::class::ClassError;
    pub use crate::combat_log::CombatLogError;
    pub use crate::entity::EntityError;
    pub use crate::field_value::FieldValueError;
    pub use crate::parser::ParserError;
    pub use crate::serializer::SerializerError;
    pub use crate::string_table::StringTableError;
}

pub mod proto {
    pub use d2_stampede_protobufs::prost::Message;
    pub use d2_stampede_protobufs::*;
}

pub use crate::parser::ParserError;

/// Result type for observers ([`anyhow::Result`])
pub type ObserverResult = anyhow::Result<()>;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
