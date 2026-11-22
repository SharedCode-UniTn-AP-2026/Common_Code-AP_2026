//! This module defines the different traits used in the channel library.

use crate::message_types::*;
use crossbeam_channel::{Receiver, Sender};
use shared::ShipId;

/// Trait that defines the behavior of a communication channel between two entities.
pub trait ComunicationChannel<T, U> {
    /// Creates a new communication channel with the specified sender and receiver.
    fn new(to: Sender<T>, from: Receiver<U>) -> Self;
    /// Returns a reference to the sender of the channel.
    fn outbox(&self) -> &Sender<T>;
    /// Returns a reference to the receiver of the channel.
    fn inbox(&self) -> &Receiver<U>;
}

/// Trait that defines helper functions to manage messages for the Ships.
#[allow(unused)]
pub trait ShipMessageManager {
    /// Sends a message to all ships.
    ///
    /// # Panics
    /// - Will panic if a channel to a ship is disconnected.
    fn ship_broadcast(&self, message: AdmiraltyToShip);
    /// Sends a message to a specific ship.
    ///
    /// # Panics
    /// - Will panic if the channel to the ship is disconnected.
    /// - Will panic if the ship_id does not exist in the channel organizer.
    fn ship_send(&self, ship_id: ShipId, message: AdmiraltyToShip);
    /// Receives a message from a specific ship.
    /// Hangs until a message is received or **the response deadline is reached**.
    ///
    /// # Panics
    /// - Will panic if the channel to the ship is disconnected.
    /// - Will panic if the ship_id does not exist in the channel organizer.
    fn ship_receive(&self, ship_id: ShipId) -> Option<ShipToAdmiralty>;
    /// Receives a message from a specific ship.
    /// Hangs until a message is received or 150ms have passed, after which it will **panic**.
    ///
    /// # Panics
    /// - Will panic if the channel to the ship is disconnected.
    /// - Will panic if the ship_id does not exist in the channel organizer.
    /// - Will panic if 150ms have passed without receiving a message from the ship.
    fn ship_receive_important(&self, ship_id: ShipId) -> ShipToAdmiralty;
}

/// Trait that defines helper functions to manage messages for the Cartographer.
#[allow(unused)]
pub trait CartographerMessageManager {
    /// Sends a message to the cartographer.
    ///
    /// # Panics
    /// - Will panic if the channel to the cartographer is disconnected.
    fn cartographer_send(&self, message: AdmiraltyToCartographer);
    /// Receives a message from the cartographer.
    /// Hangs until a message is received or 150ms have passed, after which it will **panic**.
    ///
    /// # Panics
    /// - Will panic if the channel from the cartographer is disconnected.
    /// - Will panic if 150ms have passed without receiving a message from the cartographer.
    fn cartographer_receive(&self) -> Option<CartographerToAdmiralty>;
}
