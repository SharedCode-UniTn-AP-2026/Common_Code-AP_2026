//! This crate contains the messages API/lib
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

pub mod api;
mod channels;
mod messages;

// TODO:
// - Docum
// - Tetsts
// - Change println! to logging
// - Reflect changes in protocol
// - Should WE implement a message validity checker? (Eg. verifies that Letter do not exceed 256 chars)
//      - Same goes for helper funcs such as `relay_letter`
//
//
// WAITING ON:
// - ShipView
// - SpectatorView
// - Event
// - Outcome
// - Map is not dyn compatible, options:
//      - Use generics (ugly)
//      - Make Map object safe (dyn-compatible)
//
// PROTOCOL REFLECT:
// - SunkReason was added because a ship has to know who shot it even if it dies
// - Rejected was put in RoundResult to avoid conflicts between the different TurnMode and it makes more sense generally
// - Added a Relayed letter to solve #34
// - Deadlines for the cartographer
// Added rejects TooManyClues and TooManyChars
//
//
// COMMOND RECEIVE API
// --> SMISTA MSG
// --> RETURN ...TO...
