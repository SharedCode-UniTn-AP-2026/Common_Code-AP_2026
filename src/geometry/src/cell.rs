///TODO: Missing docs for Direction
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Direction {
    ///TODO: Missing docs for Direction
    East,
    ///TODO: Missing docs for Direction
    NorthEast,
    ///TODO: Missing docs for Direction
    NorthWest,
    ///TODO: Missing docs for Direction
    West,
    ///TODO: Missing docs for Direction
    SouthWest,
    ///TODO: Missing docs for Direction
    SouthEast,
}

impl Direction {
    /// TODO: missing docs for `offset`
    pub fn offset(self) -> (i32, i32) {
        todo!();
    }
    /// TODO: missing docs for `relative_angle`
    pub fn relative_angle(self, other: Direction) -> u8 {
        todo!()
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
}
