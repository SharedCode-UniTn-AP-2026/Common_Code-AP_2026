//! This module contains the types of channels used for communication between the Admiralty, the Cartographer, and the Ships.

use crate::messages::message_types::{cartographer_messages::*, ship_messages::*};

use super::traits::ComunicationChannel;

use crossbeam_channel::{Receiver, Sender};

/// Struct that represents the channels used by the Cartographer to talk to the Admiralty.
pub struct CartographerToAdmiraltyChannel {
    to: Sender<CartographerToAdmiralty>,
    from: Receiver<AdmiraltyToCartographer>,
}

impl ComunicationChannel<CartographerToAdmiralty, AdmiraltyToCartographer> for CartographerToAdmiraltyChannel {
    fn new(to: Sender<CartographerToAdmiralty>, from: Receiver<AdmiraltyToCartographer>) -> Self {
        Self { to, from }
    }

    fn outbox(&self) -> &Sender<CartographerToAdmiralty> {
        &self.to
    }

    fn inbox(&self) -> &Receiver<AdmiraltyToCartographer> {
        &self.from
    }
}

/// Struct that represents the channels used by the Admiralty to talk to the Cartographer.
pub struct AdmiraltyToCartographerChannel {
    to: Sender<AdmiraltyToCartographer>,
    from: Receiver<CartographerToAdmiralty>,
}

impl ComunicationChannel<AdmiraltyToCartographer, CartographerToAdmiralty> for AdmiraltyToCartographerChannel {
    fn new(to: Sender<AdmiraltyToCartographer>, from: Receiver<CartographerToAdmiralty>) -> Self {
        Self { to, from }
    }

    fn outbox(&self) -> &Sender<AdmiraltyToCartographer> {
        &self.to
    }

    fn inbox(&self) -> &Receiver<CartographerToAdmiralty> {
        &self.from
    }
}
/// Struct that represents the channels used by a Ship to talk to the Admiralty.
pub struct ShipToAdmiraltyChannel {
    to: Sender<ShipToAdmiralty>,
    from: Receiver<AdmiraltyToShip>,
}

impl ComunicationChannel<ShipToAdmiralty, AdmiraltyToShip> for ShipToAdmiraltyChannel {
    fn new(to: Sender<ShipToAdmiralty>, from: Receiver<AdmiraltyToShip>) -> Self {
        Self { to, from }
    }

    fn outbox(&self) -> &Sender<ShipToAdmiralty> {
        &self.to
    }

    fn inbox(&self) -> &Receiver<AdmiraltyToShip> {
        &self.from
    }
}
/// Struct that represents the channels used by the Admiralty to talk to a Ship.
pub struct AdmiraltyToShipChannel {
    to: Sender<AdmiraltyToShip>,
    from: Receiver<ShipToAdmiralty>,
}

impl ComunicationChannel<AdmiraltyToShip, ShipToAdmiralty> for AdmiraltyToShipChannel {
    fn new(to: Sender<AdmiraltyToShip>, from: Receiver<ShipToAdmiralty>) -> Self {
        Self { to, from }
    }

    fn outbox(&self) -> &Sender<AdmiraltyToShip> {
        &self.to
    }

    fn inbox(&self) -> &Receiver<ShipToAdmiralty> {
        &self.from
    }
}
