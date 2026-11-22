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
        map: Box<dyn Map>,
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

