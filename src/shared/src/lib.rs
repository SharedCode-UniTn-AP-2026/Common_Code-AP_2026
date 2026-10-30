//! Shared protocol types for the project.

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

mod params;
mod turntoken;
pub use params::Params;
pub use turntoken::TurnToken;

///TODO: add docs
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ShipId(pub u8);
///TODO: add docs
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct IslandId(pub u8);
///TODO: add docs
pub type Round = u32;
///TODO: add docs
pub type Seed = u64;
