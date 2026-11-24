use crate::messages::message_types;

/// Trait that defines helper functions for entities that have to manage messages from/to Admiralty
#[allow(unused)]
pub trait CartographerMessageManager {
    /// Pause the game
    pub fn pause();

    /// Resume the game
    pub fn resume();

    /// Step the game
    pub fn step();

    /// Set the mode
    pub fn set_mode();

    /// Sep new deadline
    pub fn set_deadline(new_deadline: u32);

    /// Do nothing, skip the turn
    pub fn expire_turn();

    /// 
    pub fn dump();

    /// receive a complete snapshot for the Cartographer.
    pub fn receive_state() -> Result<(), SendError<AdmiraltyToCartographer>>;
}


