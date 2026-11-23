//! This module defines the different traits used in the channel library.
use crossbeam_channel::{Receiver, Sender};

/// Trait that defines the behavior of a communication channel between two entities.
pub trait ComunicationChannel<T, U> {
    /// Creates a new communication channel with the specified sender and receiver.
    fn new(to: Sender<T>, from: Receiver<U>) -> Self;
    /// Returns a reference to the sender of the channel.
    fn outbox(&self) -> &Sender<T>;
    /// Returns a reference to the receiver of the channel.
    fn inbox(&self) -> &Receiver<U>;
}
