//! Geometry types and operations shared by the project.

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

mod cell;
mod los;
mod map;
mod terrain;

pub use cell::{Cell, Direction};
pub use los::visible;
pub use map::{Error as MapError, Map};
pub use terrain::Terrain;
