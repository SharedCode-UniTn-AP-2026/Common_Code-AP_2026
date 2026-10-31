use shared::IslandId;
/// The terrain of a map cell, classified as water or land.
///
/// Water is either open (`Clear`) or rocky (`Rock`). Land either belongs to an
/// island (`Island`), identified by its island ID, or is unowned (`Cliff`).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Terrain {
    /// Open water.
    Clear,
    /// Water, but foul. Entering costs half a full hull, rounded up.
    Rock,
    /// Land, and part of an Island’s footprint.
    Island(IslandId),
    ///Land nobody owns.
    Cliff,
}

impl Terrain {
    /// Returns `true` if this terrain is [`Terrain::Island`] or [`Terrain::Cliff`].
    pub fn is_land(&self) -> bool {
        matches!(self, Terrain::Island(_) | Terrain::Cliff)
    }
    /// Returns `true` if this terrain is [`Terrain::Rock`] or [`Terrain::Clear`].
    pub fn is_water(&self) -> bool {
        matches!(self, Terrain::Rock | Terrain::Clear)
    }
}
