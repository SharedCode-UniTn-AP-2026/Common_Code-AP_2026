//! Shared protocol types for the project.

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

mod params;
mod turntoken;
pub use params::Params;
pub use turntoken::TurnToken;

/// Identifies one of the six ships in the game.
///
/// Valid game identifiers range from 0 to 5.
/// Ship `i` starts at map corner `i`.
///
/// The type itself also permits values outside this range.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct ShipId(
    /// Numeric ship identifier; valid game values are 0 through 5.
    pub u8,
);

impl ShipId {
    /// All six valid ship identifiers in ascending order.
    ///
    /// Each ship starts at the map corner with the same index.
    pub const ALL: [ShipId; 6] = [ShipId(0), ShipId(1), ShipId(2), ShipId(3), ShipId(4), ShipId(5)];
}

/// Identifies an island within a game.
///
/// This type does not guarantee that the identified island exists.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct IslandId(
    /// Numeric island identifier.
    pub u8,
);

/// Game round number.
///
/// Rounds start at 1. As an alias for `u32`, this type
/// does not prevent the representation of 0.
pub type Round = u32;

/// Seed used for reproducible random generation.
///
/// Game randomness must be derived from this seed,
/// rather than the system clock or thread-local randomness.
pub type Seed = u64;

#[cfg(test)]
mod tests {
    use super::*;

    // Checks that all six valid ship identifiers are present
    // exactly once and in ascending order.
    #[test]
    fn test_ship_id_all() {
        assert_eq!(ShipId::ALL.map(|ship| ship.0), [0, 1, 2, 3, 4, 5]);
    }
}
