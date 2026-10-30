use crate::{Cell, Terrain};

///TODO: missing documentation for Map
pub struct Map {}

///TODO: missing documentation
///
///Would be useful if Error implemented std::err::Error. See [`thiserror`](https://docs.rs/thiserror)
pub enum Error {}

impl Map {
    ///TODO: missing docs
    pub fn try_new(radius: u32, cells: impl IntoIterator<Item = (Cell, Terrain)>) -> Result<Map, Error> {
        todo!()
    }
    ///TODO: missing docs
    pub fn radius(&self) -> u32 {
        todo!()
    }
    ///TODO: missing docs
    pub fn contains(&self, cell: Cell) -> bool {
        todo!()
    }
    ///TODO: missing docs
    pub fn terrain(&self, cell: Cell) -> Option<Terrain> {
        todo!()
    }
    ///TODO: missing docs
    pub fn is_land(&self, cell: Cell) -> bool {
        todo!()
    }
    ///TODO: missing docs
    pub fn is_water(&self, cell: Cell) -> bool {
        todo!()
    }
    ///TODO: missing docs
    pub fn corners(&self) -> [Cell; 6] {
        todo!()
    }
    ///TODO: missing docs
    pub fn water_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        todo!() as std::iter::Empty<Cell>
    }
    ///TODO: missing docs
    pub fn clear_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        todo!() as std::iter::Empty<Cell>
    }
}
