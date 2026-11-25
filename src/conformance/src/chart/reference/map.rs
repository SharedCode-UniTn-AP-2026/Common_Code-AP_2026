use geometry::{Cell, Map, Terrain};
use std::collections::BTreeMap;

/// A reference [`Map`], used in [`RefChart`]
#[derive(Debug, PartialEq)]
pub struct RicMap {
    radius: u32,
    cells: BTreeMap<Cell, Terrain>,
}

impl Map for RicMap {
    fn radius(&self) -> u32 {
        self.radius
    }

    fn contains(&self, cell: Cell) -> bool {
        self.cells.get(&cell).is_some()
    }

    fn terrain(&self, cell: Cell) -> Option<Terrain> {
        self.cells.get(&cell).map(|terr| terr.clone())
    }

    fn is_land(&self, cell: Cell) -> bool {
        self.terrain(cell).is_some_and(|t| t.is_land())
    }
    fn is_water(&self, cell: Cell) -> bool {
        self.terrain(cell).is_some_and(|t| t.is_water())
    }
    /// The six corners, in direction order (counter-clockwise, East is the first). Ship i starts on corner i.
    fn corners(&self) -> [Cell; 6] {
        let rad = self.radius().cast_signed();
        [
            Cell { q: rad, r: 0 },
            Cell { q: rad, r: -rad },
            Cell { q: 0, r: -rad },
            Cell { q: -rad, r: 0 },
            Cell { q: -rad, r: rad },
            Cell { q: 0, r: rad },
        ]
    }
    /// Every cell in the sea, in a fixed order. Clear water and rocks.
    fn water_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell { q: 0, r: 0 }
            .disc(self.radius())
            .filter(|&cell| self.is_water(cell))
    }
    /// Water that is not a rock.
    fn clear_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        Cell { q: 0, r: 0 }.disc(self.radius()).filter(|&cell| {
            if let Some(terr) = self.cells.get(&cell) {
                *terr == Terrain::Clear
            } else {
                false
            }
        })
    }
}

impl RicMap {
    /// a
    #[must_use]
    pub fn init(radius: u32, cells: BTreeMap<Cell, Terrain>) -> Self {
        Self { radius, cells }
    }
}
