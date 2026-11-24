use geometry::Cell;
use shared::{Round, ShipId};

/// Information about a [`ShipId`], needed in [`Outcome`]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ShipStanding {
    /// The id of the `Ship`
    pub ship: ShipId,
    /// Number of full strength clues hold
    pub full_clues: u8,
    /// None if still afloat at game end; `Some(r)` if sunk on round r.
    pub sunk_round: Option<Round>,
}

/// The outcome of the game
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    /// A Ship declared the Treasure's cell correctly.
    Declared {
        /// The winning [`ShipId`]
        winner: ShipId,
        /// The treasure's [`Cell`]
        treasure: Cell,
    },
    /// The round limit was reached without a correct declaration.
    RoundCap {
        /// A slice containing the [`Ship`]
        ranking: [ShipStanding; 6],
    },
    /// Every Ship sank before a correct declaration or the round cap.
    AllSunk {
        /// A slice containing the [`Ship`]
        ranking: [ShipStanding; 6],
    },
}
