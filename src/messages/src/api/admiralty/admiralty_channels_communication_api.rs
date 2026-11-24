//! This module contains helper functions to help manage messages and channels

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError, TrySendError, bounded};
use std::{collections::HashMap, time::Duration};

use crate::shared::ShipId;
use crate::messages::message_types;
use crate::messages::channels;

/// Struct implemented by the Admiralty to organize communication and channels for every Ship and the cartographer.
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

impl ChannelsCreation for AdmiraltyChannelManager {
    /// Generates the tuple (Sender, Receiver) for the channel : **Admiralty -> Ship**
    fn generate_channels_admiralty_to_ship(
        channel_capacity: usize,
    ) -> (Sender<AdmiraltyToShip>, Receiver<AdmiraltyToShip>) {
        bounded::<AdmiraltyToShip>(channel_capacity)
    }

    /// Generates the tuple (Sender, Receiver) for the channel : **Ship -> Admiralty**
    fn generate_channels_ship_to_admiralty(
        channel_capacity: usize,
    ) -> (Sender<ShipToAdmiralty>, Receiver<ShipToAdmiralty>) {
        bounded::<ShipToAdmiralty>(channel_capacity)
    }

    /// Generates the tuple (Sender, Receiver) for the channel : **Admiralty -> Cartographer**
    fn generate_channels_admiralty_to_cartographer(
        channel_capacity: usize,
    ) -> (Sender<AdmiraltyToCartographer>, Receiver<AdmiraltyToCartographer>) {
        bounded::<AdmiraltyToCartographer>(channel_capacity)
    }

    /// Generates the tuple (Sender, Receiver) for the channel : **Cartographer -> Admiralty**
    fn generate_channels_cartographer_to_admiralty(
        channel_capacity: usize,
    ) -> (Sender<CartographerToAdmiralty>, Receiver<CartographerToAdmiralty>) {
        bounded::<CartographerToAdmiralty>(channel_capacity)
    }

    /// Generates the channels for each ship.
    ///
    /// # Returns
    /// Returns two hashmaps in a tuple :
    /// 1. Contains the channels used by the Admiralty to talk to each ship.
    /// 2. Contains the channels used by each ship to talk to the Admiralty.
    fn generate_ship_channels<T: Iterator<Item = ShipId>>(
        ship_ids: T,
        channel_capacity: usize,
    ) -> (
        HashMap<ShipId, ShipToAdmiraltyChannel>,
        HashMap<ShipId, AdmiraltyToShipChannel>,
    ) {
        let mut ship_to_admiralty_channels = HashMap::new();
        let mut admirality_to_ship_channels = HashMap::new();

        for ship_id in ship_ids {
            let (to_ship_sender, to_ship_receiver) = Self::generate_channels_admiralty_to_ship(channel_capacity);
            let (from_ship_sender, from_ship_receiver) = Self::generate_channels_ship_to_admiralty(channel_capacity);

            ship_to_admiralty_channels.insert(ship_id, ShipToAdmiraltyChannel::new(from_ship_sender, to_ship_receiver));
            admirality_to_ship_channels
                .insert(ship_id, AdmiraltyToShipChannel::new(to_ship_sender, from_ship_receiver));
        }

        (ship_to_admiralty_channels, admirality_to_ship_channels)
    }

    /// Generates the channels for the cartographer.
    fn generate_cartographer_channels(
        channel_capacity: usize,
    ) -> (CartographerToAdmiraltyChannel, AdmiraltyToCartographerChannel) {
        let (to_cartographer_sender, to_cartographer_receiver) =
            Self::generate_channels_admiralty_to_cartographer(channel_capacity);
        let (from_cartographer_sender, from_cartographer_receiver) =
            Self::generate_channels_cartographer_to_admiralty(channel_capacity);

        (
            CartographerToAdmiraltyChannel::new(from_cartographer_sender, to_cartographer_receiver),
            AdmiraltyToCartographerChannel::new(to_cartographer_sender, from_cartographer_receiver),
        )
    }
}

impl ChannelsModifySettings for AdmiraltyChannelManager {
    /// Updates the response deadline for the channels.
    pub fn update_deadline(&mut self, new_deadline: u64) {
        self.response_deadline = new_deadline;
    }
}
