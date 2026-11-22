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