use crate::messages::message_types;

/// Trait that defines helper functions for entities that have to manage messages from/to the Ships.
#[allow(unused)]
pub trait AdmiraltyMessageManager {
    /// Wait welcome message received by Admiralty
    fn wait_welcome(&self) -> Welcome;

    /// Sends a message to the Admiralty.
    ///
    /// Intended to be used for select the action the ship want to do
    fn submit(&self, message: ShipToAdmiralty) -> CommsOutcome<ShipToAdmiralty>;

    /// Wait RoundOpen message received by Admiralty
    ///
    /// The ship then can select the action and call submit
    fn round_open(&self) -> RoundOpen;

    /// Wait RoundResult or GameOver message received by Admiralty
    fn receive_message(&self) -> AdmiraltyToShip;
}
