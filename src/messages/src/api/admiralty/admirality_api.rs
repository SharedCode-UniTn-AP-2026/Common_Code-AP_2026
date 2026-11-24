use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, TryRecvError, TrySendError, bounded};
use shared::ShipId;
use std::{collections::HashMap, time::Duration};

use crate::messages::message_types::{cartographer_messages::*, ship_messages::*};

impl AdmiraltyChannelManager {
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
}

impl ShipMessageManager for AdmiraltyChannelManager {
    /// Sends a message to every ship.
    ///
    /// If a ship results "disconnected" it gets removed from the HashMap of memorized (ShipId, AdmiraltyToShipChannel).
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

    /// Sends a message to a ship.
    ///
    /// If a ship results "disconnected" it gets removed from the HashMap of memorized (ShipId, AdmiraltyToShipChannel).
    ///
    /// # Returns
    /// - `CommsOutcome::Success(None)` : Message sent
    /// - `CommsOutcome::Full` : Message could not be sent because the channel is full
    /// - `CommsOutcome::NonExistent` : Message could not be sent because either the ShipId has never existed or the corresponding ship has been disconnected
    fn ship_send(&mut self, ship_id: ShipId, message: AdmiraltyToShip) -> CommsOutcome<AdmiraltyToShip> {
        if let Some(channel) = self.ship_channels.get(&ship_id) {
            match channel.outbox().try_send(message) {
                Ok(_) => CommsOutcome::Success(None),
                Err(TrySendError::Full(_)) => {
                    println!("Channel to ship {:?} is full. Message dropped.", ship_id);
                    CommsOutcome::Full
                }
                Err(TrySendError::Disconnected(_)) => {
                    println!("Channel to ship {:?} is disconnected.", ship_id);
                    self.ship_channels.remove(&ship_id);
                    CommsOutcome::NonExistent
                }
            }
        } else {
            println!(
                "Tried to send message to ship with the non-existing id : {:?}. ",
                ship_id
            );
            CommsOutcome::NonExistent
        }
    }

    /// Recieves a message from a ship.
    ///
    /// If a ship results "disconnected" it gets removed from the HashMap of memorized (ShipId, AdmiraltyToShipChannel).
    ///
    /// # Returns
    /// - `CommsOutcome::Success(message)` : Message recieved
    /// - `CommsOutcome::Timeout` : Message could not be recieved because the deadline has passed
    /// - `CommsOutcome::NonExistent` : Message could not be recieved because either the ShipId has never existed or the corresponding ship has been disconnected
    fn ship_receive(&mut self, ship_id: ShipId) -> CommsOutcome<ShipToAdmiralty> {
        if let Some(channel) = self.ship_channels.get(&ship_id) {
            let inbox = channel.inbox();
            match inbox.try_recv() {
                Ok(message) => CommsOutcome::Success(message),
                Err(TryRecvError::Disconnected) => {
                    println!("Channel from ship {:?} is disconnected. ", ship_id);

                    self.ship_channels.remove(&ship_id);
                    CommsOutcome::NonExistent
                }
                Err(TryRecvError::Empty) => match inbox.recv_timeout(Duration::from_millis(self.response_deadline)) {
                    Ok(message) => CommsOutcome::Success(message),

                    Err(RecvTimeoutError::Timeout) => CommsOutcome::TimeOut,

                    Err(RecvTimeoutError::Disconnected) => {
                        println!("Channel from ship {:?} is disconnected. ", ship_id);

                        self.ship_channels.remove(&ship_id);
                        CommsOutcome::NonExistent
                    }
                },
            }
        } else {
            println!(
                "Tried to receive message from ship with the non-existing id : {:?}. ",
                ship_id
            );
            CommsOutcome::NonExistent
        }
    }
}

impl CartographerMessageManager for AdmiraltyChannelManager {
    /// Sends a message to the cartographer.
    ///
    /// # Panics
    /// - Will panic if the channel to the cartographer is disconnected.
    ///
    /// # Returns
    /// - `CommsOutcome::Success(None)` : Message sent
    /// - `CommsOutcome::Full` : Message could not be sent because the channel is full
    fn cartographer_send(&self, message: AdmiraltyToCartographer) -> CommsOutcome<AdmiraltyToCartographer> {
        match self.cartographer_channels.outbox().try_send(message) {
            Ok(_) => CommsOutcome::Success(None),
            Err(TrySendError::Full(_)) => {
                println!("Channel to cartographer is full. Message dropped.");
                CommsOutcome::Full
            }
            Err(TrySendError::Disconnected(_)) => {
                panic!("Channel to cartographer is disconnected. This should never happen.");
            }
        }
    }

    /// Receives a message from the cartographer.
    /// Hangs until a message is received or 150ms have passed, after which it will **panic**.
    ///
    /// # Panics
    /// - Will panic if the channel from the cartographer is disconnected.
    /// - Will panic if 1s has passed without receiving a message from the cartographer.
    ///
    /// # Returns
    /// - `CommsOutcome::Success(message)` : Message recieved
    fn cartographer_receive(&self) -> CommsOutcome<CartographerToAdmiralty> {
        let inbox = self.cartographer_channels.inbox();
        match inbox.try_recv() {
            Ok(message) => CommsOutcome::Success(message),
            Err(TryRecvError::Disconnected) => {
                panic!("Channel from cartographer is disconnected. This should never happen.");
            }
            Err(TryRecvError::Empty) => match inbox.recv_timeout(Duration::from_secs(1)) {
                Ok(message) => CommsOutcome::Success(message),
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
