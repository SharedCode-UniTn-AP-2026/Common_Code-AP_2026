use shared::{Round, ShipId};

/// Rejection causes for an [`Action`]
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rejection {
    ///
    WrongRound {
        ///
        open: Round,
    },
    /// [`TurnToken`] is stale
    StaleToken,
    /// [`Ship`] wanted to move to a non-adjacent [`Cell`]
    NotANeighbour,
    /// [`Ship`] wanted to move into the void
    OutsideArchipelago,
    /// [`Ship`] wanted to move into land
    CellIsLand,
    /// [`Ship`] wanted to move to a [`Cell`] occupied by some other [`Ship`]
    CellOccupied {
        /// The other [`ShipId`]
        by: ShipId,
    },
    /// [`Ship`] wanted to go against the wind
    AgainstTheWind,
    ///
    NotAdjacentToIsland,
    ///
    NoBerth,
    ///
    IslandRefused,
    /// [`Ship`] cannot ask clue if it's not moored
    NotMoored,
    /// [`Ship`] wanted to
    OutOfRange,
    /// [`Ship`] wanted to bombard something not visible
    NotVisible,
    /// [`Ship`] may not have ever existed
    NoSuchShip,
    /// [`Ship`] wanted to form an alliance (again)
    AlreadyAllied,
    ///
    NoSuchProposal,
    /// [`Ship`] wanted to break an alliance but was not allied
    NotAllied,
    /// [`Ship`] wanted to break an alliance but was not allied
    TargetHasSunk,
    ///
    TooMuchMail,
}
