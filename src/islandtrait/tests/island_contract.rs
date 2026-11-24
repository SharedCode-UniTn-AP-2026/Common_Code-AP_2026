//! Integration tests for the shared `Island` trait.
//!
//! `MockIsland` is a minimal example implementation used to exercise the interface
//! and its default methods. Its one-berth policy and three-round waiting period
//! are test-specific choices, not requirements for other island implementations.

use std::collections::HashSet;

use ap_2026_common_code_islandtrait::island::{Island, IslandInit, IslandResponse, MoorResponse};

use ap_2026_common_code_islandtrait::types::{Clue, Params, Round, ShipId};

// Minimal test implementation of the shared Island trait.
struct MockIsland {
    clue: Clue,
    moored: Option<ShipId>,
    served_full: HashSet<ShipId>,
    remaining: u32,
}
const TEST_CLUE: Clue = Clue::HalfPlane {
    axis: common_clues::Axis::Q,
    bound: common_clues::Bound::AtLeast(1),
};

impl Island for MockIsland {
    // Create a test island with a single berth and a three-round clue delay.
    fn new(init: IslandInit) -> Self {
        // No ship is moored when the island is created.
        // Each ship is tracked once it receives the full clue.
        Self {
            clue: init.clue,
            moored: None,
            served_full: HashSet::new(),
            remaining: 3,
        }
    }
    // Register a ship as moored.
    fn moor(&mut self, ship: ShipId) {
        self.moored = Some(ship);
    }
    // Check berth availability without modifying the island state.
    fn can_moor(&self, _ship: ShipId) -> MoorResponse {
        if self.moored.is_none() {
            MoorResponse::Accepted
        } else {
            MoorResponse::NoBerth
        }
    }
    // Check whether the given ship is currently moored.
    fn is_moored(&self, ship: ShipId) -> bool {
        self.moored == Some(ship)
    }
    // Release the berth only if it belongs to this ship.
    fn leave_berth(&mut self, ship: ShipId) {
        if self.moored == Some(ship) {
            self.moored = None;
        }
    }

    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        if self.served_full.contains(&ship) {
            return IslandResponse::NothingMore;
        }
        if self.remaining > 0 {
            return IslandResponse::Wait(self.remaining);
        }
        self.served_full.insert(ship);
        IslandResponse::Respond(self.clue)
    }

    fn remaining_wait(&self, _ship: ShipId) -> Option<u32> {
        Some(self.remaining)
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        !self.served_full.contains(&ship)
    }

    fn round_advance(&mut self, _round: Round) {
        if self.remaining > 0 {
            self.remaining -= 1;
        } // Once the counter reaches zero, it stays at zero.
    }
}

#[test]
fn test_island_trait_object() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut mock = MockIsland::new(init);

    let island: &mut dyn Island = &mut mock;
    let ship1 = ShipId(1);

    assert_eq!(island.can_moor(ship1), MoorResponse::Accepted);
}

#[test]
fn test_default_ask_history() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mock = MockIsland::new(init);
    let ship1 = ShipId(1);
    assert_eq!(mock.ask_history(ship1), None);
}

#[test]
fn test_moor_lifecycle() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    let ship1 = ShipId(1);
    let ship2 = ShipId(2);
    // Initially, ship 1 is not moored.
    assert!(!island.is_moored(ship1));

    // The island accepts ship 1 when the berth is free.
    assert_eq!(island.can_moor(ship1), MoorResponse::Accepted);

    // Checking mooring availability must not change the island state.
    assert!(!island.is_moored(ship1));

    // Register ship 1 as moored.
    island.moor(ship1);
    assert!(island.is_moored(ship1));

    // Ship 2 cannot moor while the berth is occupied.
    assert_eq!(island.can_moor(ship2), MoorResponse::NoBerth);

    // Ship 1 leaves the island.
    island.leave_berth(ship1);
    assert!(!island.is_moored(ship1));

    // The berth is available again.
    assert_eq!(island.can_moor(ship2), MoorResponse::Accepted);
}
#[test]
fn test_wait_for_clue() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    let ship1 = ShipId(1);
    island.moor(ship1);

    // The mock island initially requires three rounds of waiting.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(3));

    // Advance the game by one round.
    island.round_advance(1);

    // Two rounds remain before the clue becomes available.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(2));

    island.round_advance(2);
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(1));

    island.round_advance(3);

    // The waiting period has ended.
    assert_eq!(island.remaining_wait(ship1), Some(0));

    // The ship receives the full clue.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Respond(TEST_CLUE));

    // After receiving the full clue,
    // the island has nothing more to offer this ship.
    assert!(!island.has_more_to_offer(ship1));

    // A repeated request does not reveal another clue.
    assert_eq!(island.ask_clue(ship1), IslandResponse::NothingMore);
}
// Verify that the waiting counter stops at zero.
#[test]
fn test_round_advance_zero() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };
    let ship1 = ShipId(1);
    let mut island = MockIsland::new(init);

    island.moor(ship1);

    // The mock island initially requires three rounds of waiting.
    assert_eq!(island.remaining_wait(ship1), Some(3));

    island.round_advance(1);
    assert_eq!(island.remaining_wait(ship1), Some(2));

    island.round_advance(2);
    assert_eq!(island.remaining_wait(ship1), Some(1));

    island.round_advance(3);
    assert_eq!(island.remaining_wait(ship1), Some(0));

    island.round_advance(4);
    assert_eq!(island.remaining_wait(ship1), Some(0));

    island.round_advance(5);
    assert_eq!(island.remaining_wait(ship1), Some(0));
}

#[test]
fn test_multiple_ships() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let ship1 = ShipId(1);
    let ship2 = ShipId(2);

    let mut island = MockIsland::new(init);
    island.moor(ship1);

    // The mock island initially requires three rounds of waiting.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(3));

    // Advance the game by one round.
    island.round_advance(1);

    // Two rounds remain before the clue becomes available.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(2));

    island.round_advance(2);
    assert_eq!(island.ask_clue(ship1), IslandResponse::Wait(1));

    island.round_advance(3);

    // The waiting period has ended.
    assert_eq!(island.remaining_wait(ship1), Some(0));

    // The ship receives the full clue.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Respond(TEST_CLUE));
    island.leave_berth(ship1);

    island.moor(ship2);

    // Ship 2 arrives after ship 1 leaves.
    // In this mock, the waiting counter is shared and is not reset between ships.
    // The waiting period has ended.
    assert_eq!(island.remaining_wait(ship2), Some(0));

    // The ship receives the full clue.
    assert_eq!(island.ask_clue(ship2), IslandResponse::Respond(TEST_CLUE));
}
#[test]
fn test_can_moor_is_pure() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    let ship1 = ShipId(1);
    // The island accepts the mooring request.
    assert_eq!(island.can_moor(ship1), MoorResponse::Accepted);
    // A successful availability check must not moor the ship.
    assert!(!island.is_moored(ship1));
    island.moor(ship1);
    assert!(island.is_moored(ship1));
}
// Verify that leaving the berth frees it for another ship.
#[test]
fn test_leave_berth() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    let ship1 = ShipId(1);
    let ship2 = ShipId(2);
    // Moor ship 1 and verify that it occupies the berth.
    island.moor(ship1);
    assert!(island.is_moored(ship1));
    island.leave_berth(ship1);
    assert!(!island.is_moored(ship1));

    // A second ship can now moor.
    assert_eq!(island.can_moor(ship2), MoorResponse::Accepted);
    island.moor(ship2);
    assert!(island.is_moored(ship2));
}

#[test]
fn test_default_repair_hull() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    let ship1 = ShipId(1);

    // The default implementation does not offer hull repairs.
    assert!(island.repair_hull(ship1).is_none());
}

#[test]
fn test_has_more_to_offer_initial() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    let ship1 = ShipId(1);

    island.moor(ship1);

    assert!(island.has_more_to_offer(ship1));

    // Advance one round while the clue is still pending.
    island.round_advance(1);

    // The island still has a clue to offer.
    assert!(island.has_more_to_offer(ship1));
}
#[test]
fn test_leave_berth_wrong_ship() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    let ship1 = ShipId(1);
    let ship2 = ShipId(2);

    island.moor(ship1);

    // Ship 2 attempts to release a berth occupied by ship 1.
    island.leave_berth(ship2);

    // Ship 1 must remain moored.
    assert!(island.is_moored(ship1));

    // Ship 2 must not be moored.
    assert!(!island.is_moored(ship2));
}

#[test]
fn test_wait_without_asking_clue() {
    let init = IslandInit {
        clue: TEST_CLUE,
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    let ship1 = ShipId(1);

    island.moor(ship1);

    assert_eq!(island.remaining_wait(ship1), Some(3));

    // Advance three rounds without requesting a clue.
    island.round_advance(1);
    island.round_advance(2);

    island.round_advance(3);

    // The waiting period must have ended.

    assert_eq!(island.remaining_wait(ship1), Some(0));
    // The first clue request now returns the full clue.
    assert_eq!(island.ask_clue(ship1), IslandResponse::Respond(TEST_CLUE));
}
