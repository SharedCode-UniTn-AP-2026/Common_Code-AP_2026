use crate::{api::admiralty::types::CommsOutcome, messages::message_types::{cartographer_messages::*, ship_messages::*}};
use shared::ShipId;

pub struct AdmiraltyChannelManager {
    /// A HashMap that maps a ShipId to its respective channels that allow the Admiralty to communicate with that Ship.
    ship_channels: HashMap<ShipId, AdmiraltyToShipChannel>,

    /// A channel that allows the Admiralty to communicate with the Cartographer.
    cartographer_channels: AdmiraltyToCartographerChannel,

    /// Maximum time (_milliseconds_) allowed for a ship to respond to a message from the Admiralty.
    response_deadline: u64,

    /// Maximum capacity of the channels used for communication between the Admiralty and the Ships or the Cartographer.
    channel_capacity: usize,
}

/// Trait that defines helper functions for entities that have to manage messages from/to the Ships.
#[allow(unused)]
pub trait ShipMessageManager {
    /// Sends a message to all ships.
    ///
    /// Intended to be used for system messages like Welcome and GameOver
    fn ship_broadcast(&mut self, message: AdmiraltyToShip);
    /// Sends a message to a specific ship.
    fn ship_send(&mut self, ship_id: ShipId, message: AdmiraltyToShip) -> CommsOutcome<AdmiraltyToShip>;
    /// Receives a message from a specific ship.
    fn ship_receive(&mut self, ship_id: ShipId) -> CommsOutcome<ShipToAdmiralty>;
}

/// Trait that defines helper functions for entities that have to manage messages from/to the Cartographer.
#[allow(unused)]
pub trait CartographerMessageManager {
    /// Sends a message to the cartographer.
    fn cartographer_send(&self, message: AdmiraltyToCartographer) -> CommsOutcome<AdmiraltyToCartographer>;
    /// Receives a message from the cartographer.
    fn cartographer_receive(&self) -> CommsOutcome<CartographerToAdmiralty>;
}

