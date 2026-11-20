//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use geometry::Cell;
use shared::{IslandId, ShipId};

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
/// Every action that a ship can perform.
pub enum Action {
    /// A ship that moves to an adjacent cell.
    Move {
        /// The cell where the ship wants to go.
        to: Cell,
    },
    /// A ship that moors at an island.
    Moor {
        /// The id of the island where the ship will moor.
        island: IslandId,
    },
    /// A ship that asks the island where it is moored for a clue.
    ///
    /// It works only if the ship is moored at an island.
    AskClue,

    /// A ship that bombards another ship.
    ///
    /// It works only if the target is in sight.
    Bombard {
        /// The `ShipId` of the target.
        target: ShipId,
    },
    /// A ship that proposes an alliance to another ship.
    ProposeAlliance {
        /// The `ShipId` of the ship to make an alliance with.
        to: ShipId,
    },
    /// A ship that accepts an alliance.
    AcceptAlliance {
        /// The `ShipId` of the ship that proposed the alliance.
        proposer: ShipId,
    },
    /// A ship that breaks its alliance.
    ///
    /// It works only if the ship is already in an alliance.
    BreakAlliance,

    /// A ship that declares the treasure in its cell.
    Declare,

    /// A ship that does nothing.
    Hold,
}

/// The possible endings of the game for a ship.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EndReason {
    /// The ship is sunk.
    Sunk,
    /// The ship correctly declared a treasure.
    Declared,
    /// The rounds are over.
    RoundCap,
}
