use shared::ShipId;
use std::collections::BTreeSet;

///
#[deprecated(
    note = "Implement once https://github.com/SharedCode-UniTn-AP-2026/AP-archipelago-protocol/issues/65 is defined"
)]
pub enum Event {}

///
pub struct Report {
    ///
    pub to: BTreeSet<ShipId>,
    ///
    pub event: Event,
}
