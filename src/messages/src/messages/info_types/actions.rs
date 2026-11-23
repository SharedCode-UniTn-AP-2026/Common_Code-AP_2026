//! Actions that a Ship can perform during a round.
//!
//! Each Ship may submit exactly one action per round.
//! The Admiralty forwards the action to the Chart, which validates
//! its legality and applies its effects to the game state.
//!
//! Invalid actions are rejected and treated as Action::Hold.

use geometry::Cell;
use shared::{IslandId, ShipId};

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    /// Moves the Ship to a neighboring cell.
    Move {
        /// Destination cell.
        to: Cell,
    },

    /// Attempts to moor the Ship at an Island.
    Moor {
        /// Identifier of the Island where the Ship intends to moor.
        island: IslandId,
    },

    /// Requests a Clue from the Island where the Ship is moored.
    AskClue,

    /// Attempts to bombard another Ship.
    Bombard {
        /// Identifier of the Ship being targeted.
        target: ShipId,
    },

    /// Proposes an alliance to another Ship.
    ProposeAlliance {
        /// Identifier of the Ship receiving the proposal.
        to: ShipId,
    },

    /// Accepts an existing alliance proposal.
    AcceptAlliance {
        /// Identifier of the Ship that proposed the alliance.
        proposer: ShipId,
    },

    /// Breaks the Ship's current alliance.
    BreakAlliance,

    /// Declares that the Treasure is located at the Ship's current cell.
    Declare,

    /// Performs no action during the round.
    Hold,
}
