use super::{GenError, Outcome, Rejection};
use geometry::Cell;
use shared::{IslandId, Params, Seed, ShipId};
use std::fmt::Debug;

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/65 is defined"
)]
pub enum Event {}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[deprecated(note = "Wait for https://github.com/SharedCode-UniTn-AP-2026/Common_Code-AP_2026/pull/64 to be merged")]
pub enum Action {
    Move { to: Cell },
    Moor { island: IslandId },
    AskClue,
    Bombard { target: ShipId },
    ProposeAlliance { to: ShipId },
    AcceptAlliance { proposer: ShipId },
    BreakAlliance,
    Declare,
    Hold,
}

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/110 is defined"
)]
pub struct ShipView {}

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/52 is defined"
)]
pub struct SpectatorView {}

/// Trait for `Chart`
pub trait Chart: Sized {
    /// A Validated action
    type ValidatedAction: Debug;

    /// Generate the archipelago, returns a [`GenError`] if some step fails
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError>;
    /// Decide whether an action is legal. The only way to obtain a `ValidatedAction`.
    fn validate(&self, ship: ShipId, action: &Action) -> Result<Self::ValidatedAction, Rejection>;
    /// Apply an already validated action. Cannot fail.
    fn apply(&mut self, action: Self::ValidatedAction) -> Vec<Event>;
    /// End of round: the round number advances and the Islands take their turn.
    fn advance_round(&mut self) -> Vec<Event>;
    /// What a Ship is permitted to see.
    fn view_for(&self, ship: ShipId) -> ShipView;
    /// What a spectator sees: everything.
    fn spectator_view(&self) -> SpectatorView;
    /// Outcome of the game
    fn outcome(&self) -> Option<Outcome>;
}
