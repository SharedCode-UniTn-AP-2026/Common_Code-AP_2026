use shared::ShipId;

/// The possible endings of the game for a ship.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EndReason {
    /// The ship is sunk.
    Sunk(SunkReason),
    /// The ship correctly declared a treasure.
    Declared,
    /// The rounds are over.
    RoundCap,
}

/// The reason why a ship was sunk.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SunkReason {
    /// The ship hit a rock.
    HitRock,
    /// The ship was shot by another ship.
    Shot(ShipId),
}
