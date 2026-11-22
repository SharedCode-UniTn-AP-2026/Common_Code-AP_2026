//! This module defines the different traits used in the channel library.

use crate::message_types::{cartographer_messages::*, ship_messages::*};
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

/// Trait that defines helper functions for entities that have to manage messages from/to the Ships.
#[allow(unused)]
pub trait ShipMessageManager {
    /// Sends a message to all ships.
    fn ship_broadcast(&mut self, message: AdmiraltyToShip);
    /// Sends a message to a specific ship.
    ///
    fn ship_send(&mut self, ship_id: ShipId, message: AdmiraltyToShip);
    /// Receives a message from a specific ship.
    /// Hangs until a message is received or **the response deadline is reached**.
    ///
    /// # Returns
    /// Returns `None` if the channel to the ship is disconnected or not present.
    fn ship_receive(&mut self, ship_id: ShipId) -> Option<ShipToAdmiralty>;
}

/// Trait that defines helper functions for entities that have to manage messages from/to the Cartographer.
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
    /// - Will panic if 1s has passed without receiving a message from the cartographer.
    fn cartographer_receive(&self) -> Option<CartographerToAdmiralty>;
}
