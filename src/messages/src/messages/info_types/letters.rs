use clues::Clue;
use shared::{Round, ShipId};

/// Letter sent by a Ship to another Ship.
///
/// Letters are included in the outbox of a submission and are
/// relayed by the Admiralty without interpreting their contents.
///
/// Each submission may contain at most three letters.
/// Each letter may contain at most four Clues and 256 characters.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Letter {
    /// Identifier of the destination Ship.
    pub to: ShipId,

    /// Clues shared with the recipient.
    ///
    /// Clues may be fabricated or modified by the sender.
    pub clues: Vec<Clue>,

    /// Free-text message addressed to the recipient.
    pub text: String,
}

/// Letter delivered to a Ship by the Admiralty.
///
/// The Admiralty attaches the sender's identity and the round
/// number before forwarding the letter to its recipient.
///
/// The sender's identity cannot be forged by another Ship.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Delivered {
    /// Round in which the letter was sent.
    pub round: Round,

    /// Identifier of the sender Ship.
    pub from: ShipId,

    /// Clues contained in the original letter.
    pub clues: Vec<Clue>,

    /// Free-text content of the original letter.
    pub text: String,
}

/// Record of a letter relayed between two Ships.
///
/// Used by the Admiralty to report inter-Ship communications
/// to the Cartographer for observation and logging.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Relayed {
    /// Round in which the letter was sent.
    pub round: Round,

    /// Identifier of the sender Ship.
    pub from: ShipId,

    /// Identifier of the destination Ship.
    pub to: ShipId,

    /// Clues included in the letter.
    pub clues: Vec<Clue>,

    /// Free-text content of the letter.
    pub text: String,
}
