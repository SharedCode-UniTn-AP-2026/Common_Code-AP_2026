use shared::*;
use geometry::Map;

pub enum ShipToAdmiralty {
    Submit {
        round: Round,
        token: TurnToken,
        action: Action,
        outbox: Vec<Letter>,
    },
}


pub enum AdmiraltyToShip {
    Welcome {
        ship: ShipId,
        roster: Vec<ShipId>,
        params: Params,
        map: Map,
        weather_seed: Seed,
    },

    RoundOpen {
        round: Round,
        deadline_ms: u32,
        view: ShipView,
        token: TurnToken,
    },

    RoundResult {
        round: Round,
        view: ShipView,
        events: Vec<Event>,
        inbox: Vec<Delivered>,
        rejected: Vec<Rejection>,
    },

    GameOver {
        round: Round,
        reason: EndReason,
        outcome: Option<Outcome>,
    },
}



pub enum CartographerToAdmiralty {
    Pause,
    Resume,
    Step,
    SetMode { mode: TurnMode },
    SetDeadline { deadline_ms: u32 },
    ExpireTurn { ship: ShipId },
    Dump,
}


pub enum AdmiraltyToCartographer {
    State {
        round: Round,
        state: SpectatorView,
        events: Vec<Event>,
        mode: TurnMode,
        letters: Vec<Relayed>,
    },
}
