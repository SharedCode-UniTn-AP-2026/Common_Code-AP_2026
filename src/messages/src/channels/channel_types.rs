use crossbeam_channel::{Receiver, Sender};
use std::collections::BTreeMap;

use shared::ShipId;

use crate::messages::{
    AdmiraltyToCartographer,
    AdmiraltyToShip,
    CartographerToAdmiralty,
    ShipToAdmiralty,
};

/// Communication endpoints owned by a Ship.
///
/// The Ship can submit actions and receive game updates.
pub struct ShipToAdmiraltyChannel {
    /// Sends submissions to the Admiralty.
    pub tx: Sender<ShipToAdmiralty>,

    /// Receives messages from the Admiralty.
    pub rx: Receiver<AdmiraltyToShip>,
}

/// Communication endpoints owned by the Admiralty
/// for one specific Ship.
pub struct AdmiraltyToShipChannel {
    /// Sends game updates to the Ship.
    pub tx: Sender<AdmiraltyToShip>,

    /// Receives submissions from the Ship.
    pub rx: Receiver<ShipToAdmiralty>,
}

/// Communication endpoints owned by the Cartographer.
pub struct CartographerToAdmiraltyChannel {
    /// Sends control commands to the Admiralty.
    pub tx: Sender<CartographerToAdmiralty>,

    /// Receives complete game snapshots.
    pub rx: Receiver<AdmiraltyToCartographer>,
}

/// Communication endpoints owned by the Admiralty
/// for the Cartographer.
pub struct AdmiraltyToCartographerChannel {
    /// Sends complete game snapshots.
    pub tx: Sender<AdmiraltyToCartographer>,

    /// Receives control commands.
    pub rx: Receiver<CartographerToAdmiralty>,
}

/// Endpoints generated for the other components.
///
/// The harness distributes each endpoint to its owner.
pub struct GeneratedChannels {
    /// Communication endpoints for each Ship.
    pub ships: BTreeMap<ShipId, ShipToAdmiraltyChannel>,

    /// Communication endpoints for the Cartographer.
    pub cartographer: CartographerToAdmiraltyChannel,
}

