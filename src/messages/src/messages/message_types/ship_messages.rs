use crate::messages::info_types::{
    actions::*, end_reason::EndReason, events::Event, letters::*, outcome::Outcome, rejections::Rejection,
};
use geometry::Map;
use shared::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShipToAdmiralty {
    /// Submits the Ship's action and outgoing letters for the current round.
    Submit {
        /// Round number for which the submission is intended.
        round: Round,

        /// Token received in RoundOpen, used to validate the submission.
        token: TurnToken,

        /// Action the Ship intends to perform during the round.
        action: Action,

        /// Letters to be relayed to other Ships (maximum 3 per submission).
        outbox: Vec<Letter>,
    },
}

impl ShipToAdmiralty {
    pub fn check_letter_validity(&self) -> Option<Rejection> {
        match self {
            ShipToAdmiralty::Submit {
                token: _token,
                round: _round,
                action: _action,
                outbox,
            } => {
                if outbox.len() > 3 {
                    return Some(Rejection::TooMuchMail);
                }
                for letter in outbox {
                    if letter.clues.len() > 4 {
                        return Some(Rejection::TooManyClues);
                    }
                    if letter.text.len() > 256 {
                        return Some(Rejection::TooManyChars);
                    }
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmiraltyToShip {
    /// Initializes a Ship before the first round begins, this is always the first message received by a Ship.
    Welcome {
        /// Unique identifier assigned to the receiving Ship.
        ship: ShipId,

        /// List of all Ships participating in the game.
        roster: Vec<ShipId>,

        /// Game parameters defining the numerical rules.
        params: Params,
        map: Map,
        weather_seed: Seed,
    },

    /// Notifies a Ship that a new round has started.
    RoundOpen {
        /// Number of the newly opened round.
        round: Round,

        /// Time available to submit an action, in milliseconds.
        deadline_ms: u32,

        /// Current game state visible to the receiving Ship.
        view: ShipView,

        /// Unique token required to submit an action for this round.
        token: TurnToken,
    },

    /// Reports the results of a completed round.
    RoundResult {
        /// Number of the completed round.
        round: Round,

        /// Updated game state visible to the receiving Ship.
        view: ShipView,

        /// Events the receiving Ship is allowed to observe.
        events: Vec<Event>,

        /// Letters received from other Ships during the round.
        inbox: Vec<Delivered>,

        /// Rejection reasons associated with invalid submissions or actions.
        rejected: Vec<Rejection>,
    },

    /// Notifies a Ship that its participation in the game has ended, the Ship must terminate its execution after receiving it.
    GameOver {
        /// Round in which the Ship's participation ended.
        round: Round,

        /// Reason why the Ship's participation ended.
        reason: EndReason,

        /// Final game outcome, if available.
        outcome: Option<Outcome>,
    },
}
