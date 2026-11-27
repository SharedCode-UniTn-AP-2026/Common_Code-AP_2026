use clues::Clue;
use islandtrait::island::{Island, IslandInit, IslandResponse, MoorResponse, RepairHullResponse};
use shared::{IslandId, Round, Seed, ShipId};
use std::collections::BTreeMap;

pub struct ConfIsland {
    id: IslandId,
    _clue: Clue,
    _seed: Seed,
}

impl Island for ConfIsland {
    fn new(init: IslandInit) -> Self
    where
        Self: Sized,
    {
        Self {
            id: IslandId(0),
            _clue: init.clue,
            _seed: init.seed,
        }
    }

    fn moor(&mut self, ship: ShipId) {}
    fn can_moor(&self, ship: ShipId) -> MoorResponse {
        MoorResponse::Refused
    } // read-only method
    fn is_moored(&self, ship: ShipId) -> bool {
        false
    }
    fn leave_berth(&mut self, ship: ShipId) {}
    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        IslandResponse::Refused
    }
    fn remaining_wait(&self, ship: ShipId) -> Option<u32> {
        None
    }
    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        false
    }
    fn round_advance(&mut self, round: Round) {}
    fn repair_hull(&mut self, _ship: ShipId) -> Option<RepairHullResponse> {
        None
    }
    fn ask_history(&self, _requester: ShipId) -> Option<BTreeMap<ShipId, Vec<Round>>> {
        None
    }
}

impl ConfIsland {
    pub fn set_id(&mut self, id: IslandId) {
        self.id = id;
    }
}
