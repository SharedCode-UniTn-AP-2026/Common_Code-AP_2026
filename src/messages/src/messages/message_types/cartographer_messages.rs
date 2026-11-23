use crate::messages::info_types::{events::Event, letters::*};
use shared::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CartographerToAdmiralty {
    /// Pauses game execution.
    Pause,

    /// Resumes game execution after a pause.
    Resume,

    /// Advances the game by exactly one round, then pauses.
    Step,

    /// Changes the game scheduling mode.
    SetMode {
        /// Scheduling mode to activate.
        mode: TurnMode,
    },

    /// Changes the deadline for subsequent rounds.
    SetDeadline {
        /// New round deadline, in milliseconds.
        deadline_ms: u32,
    },

    /// Immediately expires the specified Ship's current turn, the Ship is treated as having submitted Action::Hold.
    ExpireTurn {
        /// Identifier of the Ship whose turn must expire.
        ship: ShipId,
    },

    /// Requests a complete snapshot of the game state.
    Dump,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmiraltyToCartographer {
    /// Reports the complete game state and events for a round.
    State {
        /// Round number associated with the reported state.
        round: Round,

        /// Complete, unfiltered view of the game state.
        state: SpectatorView,

        /// Events generated during the round.
        events: Vec<Event>,

        /// Current game scheduling mode.
        mode: TurnMode,

        /// Letters relayed between Ships during the round.
        letters: Vec<Relayed>,
    },
}
