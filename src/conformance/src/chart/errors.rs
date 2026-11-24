use thiserror::Error;

/// [`GenError::CantPlace`] variants
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GenErrItem {
    /// Island
    Island,
    /// Cliff
    Cliff,
    /// Rock
    Rock,
    /// Treasure
    Treasure,
}

/// Generator error
#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Error)]
pub enum GenError {
    /// Fails step 2
    #[error("`archipelago_radius` <= `vision_range`.")]
    VisionRangeTooHigh,
    /// Fails steps 3-5 and 7
    #[error("Cannot place `{item:?}`, the sea might be too crowded.")]
    CantPlace {
        /// The item that cannot be placed
        item: GenErrItem,
    },
    /// Fails step 6
    #[error("The water is not connected by clear water alone.")]
    WaterNotConnected,
    /// Fails step 9
    #[error("The Clues do not narrow the sea down to a single cell.")]
    Unsolvable,
}
