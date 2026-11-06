//! Public Line of Sight test vectors.
//!
//! Each case describes a map and independently established
//! expected visibility results.
//!
//! All map cells are clear water unless explicitly overridden.

use crate::{Cell, Terrain};
use shared::IslandId;

/// A cell pair with its expected visibility result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisibilityVector {
    /// Starting cell.
    pub from: Cell,

    /// Target cell.
    pub to: Cell,

    /// Maximum allowed visibility distance.
    pub vision_range: u32,

    /// Whether the target must be visible from the starting cell.
    pub expected: bool,
}

/// A reference map containing one or more visibility vectors.
#[derive(Clone, Copy, Debug)]
pub struct VisibilityCase {
    /// Identifier describing the scenario.
    pub name: &'static str,

    /// Radius of the reference map.
    pub radius: u32,

    /// Terrain replacements; every other in-map cell is clear water.
    pub terrain: &'static [(Cell, Terrain)],

    /// Expected visibility results for this map.
    pub vectors: &'static [VisibilityVector],
}

/// Reference visibility cases from Protocol §3.2.
pub const PROTOCOL_3_2_CASES: &[VisibilityCase] = &[
    VisibilityCase {
        name: "clear_clear",
        radius: 2,
        terrain: &[],
        vectors: &[VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -1 },
            vision_range: 2,
            expected: true,
        }],
    },
    VisibilityCase {
        name: "land_clear",
        radius: 2,
        terrain: &[(Cell { q: 1, r: -1 }, Terrain::Cliff)],
        vectors: &[VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -1 },
            vision_range: 2,
            expected: true,
        }],
    },
    VisibilityCase {
        name: "clear_land",
        radius: 2,
        terrain: &[(Cell { q: 1, r: 0 }, Terrain::Cliff)],
        vectors: &[VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -1 },
            vision_range: 2,
            expected: true,
        }],
    },
    VisibilityCase {
        name: "land_land",
        radius: 2,
        terrain: &[
            (Cell { q: 1, r: -1 }, Terrain::Cliff),
            (Cell { q: 1, r: 0 }, Terrain::Cliff),
        ],
        vectors: &[VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -1 },
            vision_range: 2,
            expected: false,
        }],
    },
];

/// Reference visibility cases for vision range boundaries.
pub const RANGE_CASES: &[VisibilityCase] = &[VisibilityCase {
    name: "range_boundary",
    radius: 2,
    terrain: &[],
    vectors: &[
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 0,
            expected: true,
        },
    ],
}];

/// Reference cases for blocking and non-blocking terrain.
pub const OBSTACLE_CASES: &[VisibilityCase] = &[
    VisibilityCase {
        name: "cliff_obstacle",
        radius: 2,
        terrain: &[(Cell { q: 0, r: 0 }, Terrain::Cliff)],
        vectors: &[
            VisibilityVector {
                from: Cell { q: -1, r: 0 },
                to: Cell { q: 1, r: 0 },
                vision_range: 2,
                expected: false,
            },
            VisibilityVector {
                from: Cell { q: -2, r: 0 },
                to: Cell { q: 2, r: 0 },
                vision_range: 4,
                expected: false,
            },
            VisibilityVector {
                from: Cell { q: 0, r: 0 },
                to: Cell { q: 1, r: 0 },
                vision_range: 1,
                expected: true,
            },
        ],
    },
    VisibilityCase {
        name: "rock_obstacle",
        radius: 2,
        terrain: &[(Cell { q: 0, r: 0 }, Terrain::Rock)],
        vectors: &[
            VisibilityVector {
                from: Cell { q: -1, r: 0 },
                to: Cell { q: 1, r: 0 },
                vision_range: 2,
                expected: true,
            },
            VisibilityVector {
                from: Cell { q: -2, r: 0 },
                to: Cell { q: 2, r: 0 },
                vision_range: 4,
                expected: true,
            },
            VisibilityVector {
                from: Cell { q: 0, r: 0 },
                to: Cell { q: 1, r: 0 },
                vision_range: 1,
                expected: true,
            },
        ],
    },
];

/// Reference cases for an island blocking line of sight.
pub const ISLAND_CASES: &[VisibilityCase] = &[VisibilityCase {
    name: "central_island",
    radius: 2,
    terrain: &[(Cell { q: 0, r: 0 }, Terrain::Island(IslandId(1)))],
    vectors: &[
        // The island lies between the two cells.
        VisibilityVector {
            from: Cell { q: 1, r: 0 },
            to: Cell { q: -1, r: 0 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 1, r: -1 },
            to: Cell { q: -1, r: 1 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -1 },
            to: Cell { q: 0, r: 1 },
            vision_range: 2,
            expected: false,
        },
        // The island is the destination, not an intermediate obstacle.
        VisibilityVector {
            from: Cell { q: 1, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 1, r: -1 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -1 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: true,
        },
    ],
}];

/// A clear-water cell surrounded by six land cells.
pub const SURROUNDED_CASES: &[VisibilityCase] = &[VisibilityCase {
    name: "surrounded_by_land",
    radius: 2,
    terrain: &[
        (Cell { q: 1, r: 0 }, Terrain::Cliff),
        (Cell { q: 1, r: -1 }, Terrain::Cliff),
        (Cell { q: 0, r: -1 }, Terrain::Cliff),
        (Cell { q: -1, r: 0 }, Terrain::Cliff),
        (Cell { q: -1, r: 1 }, Terrain::Cliff),
        (Cell { q: 0, r: 1 }, Terrain::Cliff),
    ],
    vectors: &[
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: 0 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -2 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 0, r: -2 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -2, r: 0 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -2, r: 2 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 0, r: 2 },
            vision_range: 2,
            expected: false,
        },
        // Adjacent land cells remain visible because endpoints do not block.
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 1, r: 0 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 1, r: -1 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 0, r: -1 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -1, r: 0 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -1, r: 1 },
            vision_range: 1,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 0, r: 1 },
            vision_range: 1,
            expected: true,
        },
        // Rotated boundary cases: both nearest candidates are land.
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 2, r: -1 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 1, r: -2 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -1, r: -1 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -2, r: 1 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: -1, r: 2 },
            vision_range: 2,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 0 },
            to: Cell { q: 1, r: 1 },
            vision_range: 2,
            expected: false,
        },
    ],
}];

/// Reference cases for cells outside the map.
///
/// According to our project's chosen behavior, visibility is false
/// when either endpoint is outside the map.
pub const OUT_OF_MAP_CASES: &[VisibilityCase] = &[VisibilityCase {
    name: "outside_map",
    radius: 2,
    terrain: &[],
    vectors: &[VisibilityVector {
        from: Cell { q: 2, r: 0 },
        to: Cell { q: 3, r: 0 },
        vision_range: 1,
        expected: false,
    }],
}];
