/// One of the six directions from a hexagonal cell to its neighbors.
///
/// Variants are listed counter-clockwise starting from [`East`](Direction::East),
/// each one 60° from the previous, so their discriminants go from `0` to `5`
/// in that order. [`Direction::offset`] gives the axial `(dq, dr)` step of each one.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Direction {
    /// Offset `(1, 0)`: `q` increases.
    East,
    /// Offset `(1, -1)`: `q` increases, `r` decreases.
    NorthEast,
    /// Offset `(0, -1)`: `r` decreases.
    NorthWest,
    /// Offset `(-1, 0)`: `q` decreases.
    West,
    /// Offset `(-1, 1)`: `q` decreases, `r` increases.
    SouthWest,
    /// Offset `(0, 1)`: `r` increases.
    SouthEast,
}

impl Direction {
    /// Return the axial offset `(dq, dr)` of one step in this direction.
    ///
    /// Adding it to a cell's `(q, r)` gives the neighbour in that direction,
    /// as [`Cell::neighbour`] does.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Direction;
    ///
    /// assert_eq!(Direction::NorthEast.offset(), (1, -1));
    /// ```
    #[must_use]
    pub fn offset(self) -> (i32, i32) {
        match self {
            Direction::East => (1, 0),
            Direction::NorthEast => (1, -1),
            Direction::NorthWest => (0, -1),
            Direction::West => (-1, 0),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (0, 1),
        }
    }
    /// Return the angle between `self` and `other`, counted in 60° steps.
    ///
    /// The result is always between `0` (same direction) and `3`
    /// (opposite directions), and it is
    /// symmetric: `a.relative_angle(b) == b.relative_angle(a)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Direction;
    ///
    /// // East and SouthWest are 120° apart (going clockwise).
    /// assert_eq!(Direction::East.relative_angle(Direction::SouthWest), 2);
    /// ```
    #[must_use]
    pub fn relative_angle(self, other: Direction) -> u8 {
        let diff = (self as u8).abs_diff(other as u8);
        diff.min(6 - diff)
    }
}

/// A hexagonal cell identified by its axial coordinates `(q, r)`
///
/// The third cube coordinate is not stored: it is always `s = -q - r`
/// and can be obtained with [`Cell::s`].
///
/// Any pair of `i32` is a valid `Cell`; whether it belongs to a map is
/// decided by [`Map::contains`](crate::Map::contains).
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Cell {
    /// Axial column coordinate; increases going [`East`](Direction::East).
    pub q: i32,
    /// Axial row coordinate; increases going [`SouthEast`](Direction::SouthEast).
    pub r: i32,
}

impl Cell {
    /// Return the `s` coordinate: `s = -q -r`
    ///
    /// # Panics
    ///
    /// Panics if `q + r < -i32::MAX` or `q + r > i32::MAX`
    ///
    /// This never happens for a cell inside a [`Map`](crate::Map)
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Cell;
    ///
    /// assert_eq!(Cell { q: 2, r: -1 }.s(), -1);
    /// ```
    #[must_use]
    pub fn s(self) -> i32 {
        self.q
            .checked_add(self.r)
            .and_then(i32::checked_neg)
            .expect("s = -(q + r) overflows i32")
    }

    /// Return one cell adjacent to `self` in `direction`.
    ///
    /// # Panics
    ///
    /// Panics if `q + dq` or `r + dr` overflows `i32`, where
    /// `(dq, dr)` is [`direction.offset()`](Direction::offset).
    ///
    /// This can only happen when `q` or `r` is `i32::MIN` or `i32::MAX`,
    /// so it never happens for a cell inside a [`Map`](crate::Map).
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::{Cell, Direction};
    ///
    /// let c = Cell { q: 2, r: -1 };
    /// assert_eq!(c.neighbour(Direction::East), Cell { q: 3, r: -1 });
    /// ```
    #[must_use]
    pub fn neighbour(self, direction: Direction) -> Cell {
        let (dq, dr) = direction.offset();
        Cell {
            q: self.q.checked_add(dq).expect("neighbour q overflows i32"),
            r: self.r.checked_add(dr).expect("neighbour r overflows i32"),
        }
    }

    /// Return the distance from `self` to `other` cell.
    ///
    /// # Panics
    ///
    /// Panics if [`s`](Cell::s) panics or if `dq + dr + ds > u32::MAX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Cell;
    ///
    /// let a = Cell { q: 0, r: 0 };
    /// let b = Cell { q: 2, r: -1 };
    /// assert_eq!(a.distance(b), 2);
    /// ```
    #[must_use]
    pub fn distance(self, other: Cell) -> u32 {
        // calculating Delta q / r / s
        let dq = self.q.abs_diff(other.q);
        let dr = self.r.abs_diff(other.r);
        let ds = self.s().abs_diff(other.s());

        dq.checked_add(dr)
            .and_then(|sum| sum.checked_add(ds))
            .expect("distance overflow u32")
            / 2
    }

    /// Return the six cells adjacent to `self`.
    ///
    /// The cells are in the same order as the [`Direction`] variants:
    /// `East`, `NorthEast`, `NorthWest`, `West`, `SouthWest`, `SouthEast`.
    ///
    /// # Panics
    ///
    /// Panics if any call to [`neighbour`](Cell::neighbour) panics, which
    /// never happens for a cell inside a [`Map`](crate::Map).
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Cell;
    ///
    /// let c = Cell { q: 0, r: 0 };
    /// assert_eq!(
    ///     c.neighbours(),
    ///     [
    ///         Cell { q: 1, r: 0 },  // East
    ///         Cell { q: 1, r: -1 }, // NorthEast
    ///         Cell { q: 0, r: -1 }, // NorthWest
    ///         Cell { q: -1, r: 0 }, // West
    ///         Cell { q: -1, r: 1 }, // SouthWest
    ///         Cell { q: 0, r: 1 },  // SouthEast
    ///     ]
    /// );
    /// ```
    #[must_use]
    pub fn neighbours(self) -> [Cell; 6] {
        [
            self.neighbour(Direction::East),
            self.neighbour(Direction::NorthEast),
            self.neighbour(Direction::NorthWest),
            self.neighbour(Direction::West),
            self.neighbour(Direction::SouthWest),
            self.neighbour(Direction::SouthEast),
        ]
    }

    /// Return an iterator over all the cells **within** distance `radius` of
    /// `self`, including `self` itself.
    ///
    /// The iterator yields `3 * radius * (radius + 1) + 1` cells, which
    /// form a hexagon. Cells are yielded in ascending order (by `q`, then
    /// by `r`).
    ///
    /// # Panics
    ///
    /// - Panics immediately if `radius > i32::MAX`.
    /// - Panics during iteration if a coordinate of a yielded cell
    ///   overflows `i32`. This never happens for a cell inside a
    ///   [`Map`](crate::Map).
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Cell;
    ///
    /// let center = Cell { q: 0, r: 0 };
    ///
    /// assert_eq!(center.disc(0).collect::<Vec<_>>(), vec![center]);
    ///
    /// let disc: Vec<Cell> = center.disc(1).collect();
    /// assert_eq!(disc.len(), 7);
    /// assert!(disc.iter().all(|c| center.distance(*c) <= 1));
    /// ```
    pub fn disc(self, radius: u32) -> impl Iterator<Item = Cell> {
        let r = i32::try_from(radius).expect("disc radius overflows i32");

        (-r..=r).flat_map(move |dq| {
            // Lower and upper bounds of `dr`, so that the result is a
            // hexagon and not a parallelogram.
            let (lo, hi) = if dq >= 0 { (-r, r - dq) } else { (-r - dq, r) };

            (lo..=hi).map(move |dr| Cell {
                q: self.q.checked_add(dq).expect("disc q overflows i32"),
                r: self.r.checked_add(dr).expect("disc r overflows i32"),
            })
        })
    }

    /// Return an iterator over all the cells at distance exactly `radius`
    /// from `self`.
    ///
    /// For `radius == 0` the only cell yielded is `self`; otherwise the
    /// iterator yields `6 * radius` cells. Cells are yielded in ascending
    /// order (by `q`, then by `r`).
    ///
    /// # Panics
    ///
    /// - Panics immediately if `radius > i32::MAX`.
    /// - Panics (in debug builds) during iteration if a coordinate of a
    ///   yielded cell overflows `i32`. This never happens for a cell inside
    ///   a [`Map`](crate::Map).
    ///
    /// # Examples
    ///
    /// ```
    /// use ap_2026_common_code_geometry::Cell;
    ///
    /// let center = Cell { q: 0, r: 0 };
    ///
    /// assert_eq!(center.ring(0).collect::<Vec<_>>(), vec![center]);
    ///
    /// let ring: Vec<Cell> = center.ring(2).collect();
    /// assert_eq!(ring.len(), 12);
    /// assert!(ring.iter().all(|c| center.distance(*c) == 2));
    /// ```
    pub fn ring(self, radius: u32) -> impl Iterator<Item = Cell> {
        let r = i32::try_from(radius).expect("ring radius overflows i32");

        (-r..=r).flat_map(move |dq| {
            let (lo, hi) = if dq >= 0 { (-r, r - dq) } else { (-r - dq, r) };
            let step = if dq.unsigned_abs() == r.unsigned_abs() {
                // On the edge columns (|dq| == r) every cell is in the ring.
                1
            } else {
                // Otherwise only the two endpoints, `lo` and `hi`, are in the ring.
                lo.abs_diff(hi) as usize
            };
            (lo..=hi).step_by(step).map(move |dr| Cell {
                q: self.q + dq,
                r: self.r + dr,
            })
        })
    }
}
