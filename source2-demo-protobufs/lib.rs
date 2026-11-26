#![doc = include_str!("README.md")]

mod common;
pub use common::*;

#[cfg(feature = "dota")]
mod dota;
#[cfg(feature = "dota")]
pub use dota::*;

#[cfg(feature = "citadel")]
mod citadel;
#[cfg(feature = "citadel")]
pub use citadel::*;

#[cfg(feature = "cs2")]
mod cs2;
#[cfg(feature = "cs2")]
pub use cs2::*;

pub use prost;

#[cfg(feature = "reflection")]
pub mod schema;
