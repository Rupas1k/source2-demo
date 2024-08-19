#[cfg(all(feature = "dota", feature = "citadel"))]
compile_error!("Mutually exclusive features");

#[cfg(feature = "dota")]
mod dota;
#[cfg(feature = "dota")]
pub use dota::*;

#[cfg(feature = "citadel")]
mod citadel;
#[cfg(feature = "citadel")]
pub use citadel::*;

pub use prost;
