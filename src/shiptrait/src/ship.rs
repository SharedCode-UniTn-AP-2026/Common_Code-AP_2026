use crossbeam_channel::{Receiver, Sender};

pub trait Ship {
    fn new(rx: Receiver<AdmiraltyToShip>, tx: Sender<ShipToAdmiralty>) -> Self;

    fn run(&mut self);
}
