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

#[macro_export]
macro_rules! try_observers {
    ($self:ident, $method:ident ( $($arg:expr),* )) => {
        $self.observers
            .iter()
            .try_for_each(|obs| obs.borrow_mut().$method($($arg),*))
    };
}

pub mod prelude {
    pub use crate::{property, try_property};

    pub use crate::parser::{Context, Observer, Parser};

    pub use crate::entity::{Entity, EntityEvents};

    pub use crate::combat_log::CombatLog;

    pub use d2_stampede_protobufs::prost::Message;
}

pub use crate::parser::{Context, Observer, Parser};

pub use crate::entity::{Entity, EntityEvents};

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
