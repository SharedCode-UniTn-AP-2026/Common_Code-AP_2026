//! This module contains helper functions to help manage messages and channels

use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender, bounded};

//TEMP TYPES
///TMP
#[allow(unused)]
pub enum AdmiralityToShip {}

///TMP
#[allow(unused)]
pub enum ShipToAdmirality {}

///TMP
#[allow(unused)]
pub enum AdmiralityToCartographer {}

///TMP
#[allow(unused)]
pub enum CartographerToAdmirality {}

///TMP
pub struct ShipId {}

//^ DELETE THIS ^

/// Generates the tuple (Sender, Receiver) for the channel : **Admiralty -> Ship**
pub fn generate_channels_admiralty_to_ship(
    channel_capacity: usize,
) -> (Sender<AdmiralityToShip>, Receiver<AdmiralityToShip>) {
    bounded::<AdmiralityToShip>(channel_capacity)
}

/// Generates the tuple (Sender, Receiver) for the channel : **Ship -> Admiralty**
pub fn generate_channels_ship_to_admiralty(
    channel_capacity: usize,
) -> (Sender<ShipToAdmirality>, Receiver<ShipToAdmirality>) {
    bounded::<ShipToAdmirality>(channel_capacity)
}

/// Generates the tuple (Sender, Receiver) for the channel : **Admiralty -> Cartographer**
pub fn generate_channels_admiralty_to_cartographer(
    channel_capacity: usize,
) -> (Sender<AdmiralityToCartographer>, Receiver<AdmiralityToCartographer>) {
    bounded::<AdmiralityToCartographer>(channel_capacity)
}

/// Generates the tuple (Sender, Receiver) for the channel : **Cartographer -> Admiralty**
pub fn generate_channels_cartographer_to_admiralty(
    channel_capacity: usize,
) -> (Sender<CartographerToAdmirality>, Receiver<CartographerToAdmirality>) {
    bounded::<CartographerToAdmirality>(channel_capacity)
}

/// Struct implemented by the Admiralty to organize ship channels.
///
///
pub struct ChannelOrganizer {
    /// A HashMap that maps a ShipId to a tuple of channels.
    ///
    /// The tuple is defined as (Sender, Receiver) where the sender is **Admiralty -> Ship** and the reciever **Ship -> Admiralty**
    pub channels: HashMap<ShipId, (Sender<AdmiralityToShip>, Receiver<ShipToAdmirality>)>,

    /// Maximum time (milliseconds) allowed for a ship to respond to a message from the Admiralty.
    pub response_deadline: u32,

    //pub channel_capacity
}
