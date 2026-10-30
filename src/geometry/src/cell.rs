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

/// TODO: missing docs for `Cell`
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Cell {
    /// TODO: missing docs for `Cell`
    pub q: i32,
    /// TODO: missing docs for `Cell`
    pub r: i32,
}

impl Cell {
    /// TODO: missing docs for `Cell`
    pub fn s(self) -> i32 {
        todo!()
    }
    /// TODO: missing docs for `Cell`
    pub fn neighbour(self, direction: Direction) -> Cell {
        todo!()
    }
    /// TODO: missing docs for `Cell`
    pub fn distance(self, other: Cell) -> u32 {
        todo!()
    }
}
