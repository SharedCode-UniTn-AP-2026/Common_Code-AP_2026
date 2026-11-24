use super::rejection::Rejection;
use geometry::Cell;
use shared::{IslandId, Params, Seed, ShipId};

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/88 is defined"
)]
pub enum GenError {}

#[deprecated(note = "Create issue")]
pub enum ValidatedAction {}

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/65 is defined"
)]
pub enum Event {}

#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/111 is defined"
)]
pub enum Outcome {}

#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

pub trait Chart {
    fn generate(seed: Seed, params: &Params) -> Result<Self, GenError>
    where
        Self: Sized;
    /// Decide whether an action is legal. The only way to obtain a `ValidatedAction`.
    fn validate(&self, ship: ShipId, action: &Action) -> Result<ValidatedAction, Rejection>;
    /// Apply an already validated action. Cannot fail.
    fn apply(&mut self, action: ValidatedAction) -> Vec<Event>;
    /// End of round: the round number advances and the Islands take their turn.
    fn advance_round(&mut self) -> Vec<Event>;
    /// What a Ship is permitted to see.
    fn view_for(&self, ship: ShipId) -> ShipView;
    /// What a spectator sees: everything.
    fn spectator_view(&self) -> SpectatorView;
    fn outcome(&self) -> Option<Outcome>;
}
