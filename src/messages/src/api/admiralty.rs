//! This module contains helper functions to help manage messages and channels

pub mod traits;
pub mod types;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError, TrySendError, bounded};
use shared::ShipId;
use std::{collections::HashMap, time::Duration};
use traits::*;
use types::*;

use crate::channels::{traits::*, types::*};
use crate::messages::message_types::{cartographer_messages::*, ship_messages::*};

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

impl AdmiraltyChannelManager {
    /// Creates a new ChannelOrganizer with the specified response deadline and channel capacity and creates the channels to the cartographer.
    ///
    /// **Returns a tuple (ChannelOrganizer, Sender<CartographerToAdmiralty>, Receiver<AdmiraltyToCartographer>)**
    pub fn new<T: Iterator<Item = ShipId>>(
        response_deadline: u64,
        channel_capacity: usize,
        ship_ids: T,
    ) -> (Self, GeneratedChannels) {
        let (cartographer_to_admirality_channel, admirality_to_cartographer_channel) =
            Self::generate_cartographer_channels(channel_capacity);

        let (ship_to_admirality_channels, admirality_to_ship_channels) =
            Self::generate_ship_channels(ship_ids, channel_capacity);

        let channel_organizer = Self {
            ship_channels: admirality_to_ship_channels,
            cartographer_channels: admirality_to_cartographer_channel,
            response_deadline,
            channel_capacity,
        };

        let generated_channels = GeneratedChannels {
            ships: ship_to_admirality_channels,
            cartographer: cartographer_to_admirality_channel,
        };

        (channel_organizer, generated_channels)
    }

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

    /// Updates the response deadline for the channels.
    pub fn update_deadline(&mut self, new_deadline: u64) {
        self.response_deadline = new_deadline;
    }
}

impl ShipMessageManager for AdmiraltyChannelManager {
    fn ship_broadcast(&mut self, message: AdmiraltyToShip) {
        let mut disconneted_ships: Vec<ShipId> = vec![];
        for (id, channel) in &self.ship_channels {
            match channel.outbox().try_send(message.clone()) {
                Ok(_) => {}
                Err(e) => match e {
                    TrySendError::Full(_) => {
                        //Change to logging
                        println!("Channel to ship {:?} is full. Message dropped.", id);
                    }
                    TrySendError::Disconnected(_) => {
                        println!("Channel to ship {:?} is disconnected.", id);
                        // Remove the ship from the channel organizer if the channel is disconnected
                        disconneted_ships.push(*id);
                    }
                },
            }
        }
        for id in disconneted_ships {
            self.ship_channels.remove(&id);
        }
    }

    fn ship_send(&mut self, ship_id: ShipId, message: AdmiraltyToShip) {
        if let Some(channel) = self.ship_channels.get(&ship_id) {
            match channel.outbox().try_send(message) {
                Ok(_) => {}
                Err(TrySendError::Full(_)) => {
                    println!("Channel to ship {:?} is full. Message dropped.", ship_id);
                }
                Err(TrySendError::Disconnected(_)) => {
                    println!("Channel to ship {:?} is disconnected.", ship_id);
                    self.ship_channels.remove(&ship_id);
                }
            }
        } else {
            println!(
                "Tried to send message to ship with the non-existing id : {:?}. ",
                ship_id
            );
            self.ship_channels.remove(&ship_id);
        }
    }

    fn ship_receive(&mut self, ship_id: ShipId) -> Option<ShipToAdmiralty> {
        if let Some(channel) = self.ship_channels.get(&ship_id) {
            let inbox = channel.inbox();
            match inbox.try_recv() {
                Ok(message) => Some(message),
                Err(TryRecvError::Disconnected) => {
                    println!("Channel from ship {:?} is disconnected. ", ship_id);

                    self.ship_channels.remove(&ship_id);
                    None
                }
                Err(TryRecvError::Empty) => match inbox.recv_timeout(Duration::from_millis(self.response_deadline)) {
                    Ok(msg) => Some(msg),

                    Err(RecvTimeoutError::Timeout) => None,

                    Err(RecvTimeoutError::Disconnected) => {
                        println!("Channel from ship {:?} is disconnected. ", ship_id);

                        self.ship_channels.remove(&ship_id);
                        None
                    }
                },
            }
        } else {
            println!(
                "Tried to receive message from ship with the non-existing id : {:?}. ",
                ship_id
            );

            self.ship_channels.remove(&ship_id);
            None
        }
    }
}

impl CartographerMessageManager for AdmiraltyChannelManager {
    fn cartographer_send(&self, message: AdmiraltyToCartographer) {
        match self.cartographer_channels.outbox().try_send(message) {
            Ok(_) => {}
            Err(TrySendError::Full(_)) => {
                println!("Channel to cartographer is full. Message dropped.");
            }
            Err(TrySendError::Disconnected(_)) => {
                panic!("Channel to cartographer is disconnected. This should never happen.");
            }
        }
    }

    fn cartographer_receive(&self) -> Option<CartographerToAdmiralty> {
        let inbox = self.cartographer_channels.inbox();
        match inbox.try_recv() {
            Ok(message) => Some(message),
            Err(TryRecvError::Disconnected) => {
                panic!("Channel from cartographer is disconnected. This should never happen.");
            }
            Err(TryRecvError::Empty) => match inbox.recv_timeout(Duration::from_secs(1)) {
                Ok(msg) => Some(msg),
                Err(RecvTimeoutError::Timeout) => {
                    panic!("Message from cartographer was not received within 1s.");
                }
                Err(RecvTimeoutError::Disconnected) => {
                    panic!("Channel from cartographer is disconnected. This should never happen.");
                }
            },
        }
    }
}
