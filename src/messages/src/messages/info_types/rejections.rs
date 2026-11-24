use shared::{Round, ShipId};

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rejection {
    WrongRound { open: Round },
    StaleToken,
    NotANeighbour,
    OutsideArchipelago,
    CellIsLand,
    CellOccupied { by: ShipId },
    AgainstTheWind,
    NotAdjacentToIsland,
    NoBerth,
    IslandRefused,
    NotMoored,
    OutOfRange,
    NotVisible,
    NoSuchShip,
    AlreadyAllied,
    NoSuchProposal,
    NotAllied,
    TargetHasSunk,
    TooMuchMail,
    TooManyClues,
    TooManyChars,
}
