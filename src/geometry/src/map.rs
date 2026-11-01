use crate::{Cell, Terrain};
use std::cmp::max;
use thiserror::Error;

/// A complete hexagonal map centered on `Cell { q: 0, r: 0 }`.
///
/// A map of radius `R` contains exactly `3 * R * (R + 1) + 1` cells,
/// each with one terrain value. Its cells satisfy `|q| <= R`, `|r| <= R`
/// and `|s| <= R`, where `s = -q - r`. Construction validates these
/// invariants and stores cells in ascending order by `q`, then `r`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map {
    radius: u32,
    starts: Vec<usize>,
    terrain: Vec<(Cell, Terrain)>,
}

/// A validation failure returned by [`Map::try_new`].
///
/// Validation checks the radius, cell count, duplicates and boundaries in that
/// order, returning the first failure. The coordinate radius limit is checked
/// before consuming the input; the cell-count arithmetic is checked afterward.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    /// The radius exceeds `i32::MAX / 2`, or the expected cell count
    /// cannot be computed using `u32` arithmetic.
    #[error("invalid map radius")]
    ErrRadius,
    /// The input count differs from `3 * radius * (radius + 1) + 1`.
    /// This includes both too few and too many cells.
    #[error("wrong number of map cells")]
    ErrNumCell,
    /// At least two input entries have the same cell coordinates.
    #[error("duplicate map cells")]
    ErrCellDup,
    /// An input cell has `|q|`, `|r|` or `|s|` greater than the radius.
    #[error("cell outside map boundaries")]
    ErrCellOut,
}

impl Map {
    /// Return the starting terrain index of each column, from `q = -radius`
    /// through `q = radius`, assuming ascending `(q, r)` storage order.
    ///
    /// Requires the radius and cell count to pass [`Map::try_new`] validation.
    ///
    /// # Panics
    ///
    /// Panics if a computed column length cannot be represented as `usize`.
    /// In builds with overflow checks, also panics if coordinate or index
    /// arithmetic overflows. These conditions cannot occur for radii and cell
    /// counts validated by [`Map::try_new`].
    fn start(radius: u32) -> Vec<usize> {
        let mut starts = Vec::new();
        let mut index = 0;
        let rad = radius.cast_signed();
        for q in -rad..=rad {
            starts.push(index);
            let r_min = (-rad).max(-rad - q);
            let r_max = rad.min(rad - q);
            index += usize::try_from(r_max - r_min + 1).expect("Column length must be non negative");
        }
        starts
    }

    /// Detects every duplicate when `terrain` is sorted by `(q, r)`.
    fn check_duplicates(terrain: &[(Cell, Terrain)]) -> bool {
        terrain.windows(2).any(|x| {
            let first = x[0];
            let second = x[1];
            first.0 == second.0
        })
    }

    /// Return whether any of the first `expected_len` cells lies outside
    /// the hexagon centered on the origin with the given radius.
    ///
    /// # Panics
    ///
    /// Panics if `expected_len` exceeds `terrain.len()` or if a checked cell
    /// passes the `q` and `r` bounds but [`Cell::s`] overflows. The latter
    /// cannot occur for radii accepted by [`Map::try_new`].
    fn check_out_of_map(expected_len: usize, terrain: &[(Cell, Terrain)], radius: u32) -> bool {
        for (cell, _) in terrain.iter().take(expected_len) {
            if !(cell.q.unsigned_abs() <= radius && cell.r.unsigned_abs() <= radius) {
                return true;
            }
            let s = cell.s();
            if s.unsigned_abs() > radius {
                return true;
            }
        }
        false
    }

    /// Return the terrain index of an in-map cell.
    /// Requires [`Map::contains`] to return `true` for the given cell.
    ///
    /// # Panics
    ///
    /// Panics if the map radius cannot be represented as `i32`, the computed
    /// column index or cell offset cannot be represented as `usize`, or the
    /// column index is outside `self.starts`. In builds with overflow checks,
    /// also panics if coordinate or index arithmetic overflows. These conditions
    /// cannot occur for maps constructed by [`Map::try_new`] when
    /// [`Map::contains`] returns `true` for the cell.
    fn cell_index(&self, cell: Cell) -> usize {
        let Cell { q, r } = cell;
        let radius = i32::try_from(self.radius).expect("Map radius must fit in i32");
        let column = usize::try_from(q + radius).expect("Column index must be non-negative");
        let offset = usize::try_from(r - max(-radius, -radius - q)).expect("Cell offset must be non-negative");
        self.starts[column] + offset
    }
    /// Create a complete map from cell/terrain pairs in any input order.
    ///
    /// Each cell within `radius` of the origin must appear exactly once.
    /// Radius zero requires only the origin. Entries are stored in ascending
    /// order by `q`, then `r`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ErrRadius`] if `radius > i32::MAX / 2` or computing
    /// `3 * radius * (radius + 1) + 1` overflows `u32`;
    /// [`Error::ErrNumCell`] if the input has a different number of entries;
    /// [`Error::ErrCellDup`] if coordinates repeat; or
    /// [`Error::ErrCellOut`] if a cell lies outside the map. Checks run in
    /// that order, except that the input is collected before the cell-count
    /// arithmetic is checked.
    ///
    /// # Panics
    ///
    /// May panic if the supplied iterator panics while being converted or consumed.
    pub fn try_new(radius: u32, cells: impl IntoIterator<Item = (Cell, Terrain)>) -> Result<Map, Error> {
        if radius > (i32::MAX / 2) as u32 {
            // prevents overflow in start(), where operations like R - q can reach 2 * R
            return Err(Error::ErrRadius);
        }

        let mut terrain: Vec<_> = cells.into_iter().collect();
        //the formula for the expected len is: (3 * radius * (radius + 1) + 1) as usize;
        let expected_len = radius
            .checked_add(1)
            .and_then(|n| n.checked_mul(radius))
            .and_then(|n| n.checked_mul(3))
            .and_then(|n| n.checked_add(1))
            .ok_or(Error::ErrRadius)? as usize;

        if terrain.len() != expected_len {
            return Err(Error::ErrNumCell);
        }

        terrain.sort_by_key(|(cell, _)| (cell.q, cell.r));

        if Self::check_duplicates(&terrain) {
            return Err(Error::ErrCellDup);
        }

        if Self::check_out_of_map(expected_len, &terrain, radius) {
            return Err(Error::ErrCellOut);
        }

        let starts = Self::start(radius);

        Ok(Map {
            radius,
            starts,
            terrain,
        })
    }
    /// Return the map's radius in hexagonal cell steps from the origin.
    #[must_use]
    pub fn radius(&self) -> u32 {
        self.radius
    }
    /// Return whether `cell` lies within the map, including its boundary.
    ///
    /// Accepts any `i32` coordinates; out-of-range coordinates return `false`.
    #[must_use]
    pub fn contains(&self, cell: Cell) -> bool {
        let Cell { q, r } = cell;

        if !(q.unsigned_abs() <= self.radius && r.unsigned_abs() <= self.radius) {
            return false;
        }
        let s = cell.s();
        s.unsigned_abs() <= self.radius
    }
    /// Return the terrain at `cell`, or `None` when it lies outside the map.
    #[must_use]
    pub fn terrain(&self, cell: Cell) -> Option<Terrain> {
        if !self.contains(cell) {
            return None;
        }

        let index = self.cell_index(cell);
        Some(self.terrain[index].1)
    }
    /// Return whether `cell` is land ([`Terrain::Island`] or [`Terrain::Cliff`]).
    /// Returns `false` for cells outside the map.
    #[must_use]
    pub fn is_land(&self, cell: Cell) -> bool {
        if !self.contains(cell) {
            return false;
        }
        self.terrain[self.cell_index(cell)].1.is_land()
    }
    /// Return whether `cell` is water ([`Terrain::Clear`] or [`Terrain::Rock`]).
    /// Returns `false` for cells outside the map.
    #[must_use]
    pub fn is_water(&self, cell: Cell) -> bool {
        if !self.contains(cell) {
            return false;
        }
        self.terrain[self.cell_index(cell)].1.is_water()
    }
    /// Return the six corner cells in counter-clockwise order, starting east.
    /// The order is east, northeast, northwest, west, southwest, southeast.
    #[must_use]
    pub fn corners(&self) -> [Cell; 6] {
        let rad = self.radius.cast_signed();
        [
            Cell { q: rad, r: 0 },
            Cell { q: rad, r: -rad },
            Cell { q: 0, r: -rad },
            Cell { q: -rad, r: 0 },
            Cell { q: -rad, r: rad },
            Cell { q: 0, r: rad },
        ]
    }
    /// Iterate over all water cells ([`Terrain::Clear`] and [`Terrain::Rock`]).
    /// Cells are yielded in ascending order by `q`, then `r`.
    pub fn water_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.terrain
            .iter()
            .filter(|(_, terrain)| terrain.is_water())
            .map(|(cell, _)| *cell)
    }
    /// Iterate over all cells whose terrain is [`Terrain::Clear`].
    /// Cells are yielded in ascending order by `q`, then `r`.
    pub fn clear_cells(&self) -> impl Iterator<Item = Cell> + '_ {
        self.terrain
            .iter()
            .filter(|(_, terrain)| matches!(terrain, Terrain::Clear))
            .map(|(cell, _)| *cell)
    }
}
