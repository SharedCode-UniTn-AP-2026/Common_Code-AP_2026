use shared::ShipId;
use std::collections::BTreeSet;

///
#[deprecated(note = "Implement once `Event` has been defined")]
pub enum Event {}

/// A report to be delivered to [`Ship`] at the end of each turn, or after an action
pub struct Report {
    /// The list of [`ShipId`]
    pub to: BTreeSet<ShipId>,
    /// The [`Event`] to be delivered
    pub event: Event,
}
