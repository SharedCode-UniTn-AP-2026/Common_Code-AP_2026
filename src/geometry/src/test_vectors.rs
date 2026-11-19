//! Public Line of Sight test vectors.
//!
//! Each case describes a map and independently established
//! expected visibility results.
//!
//! All map cells are clear water unless explicitly overridden.

use crate::{Cell, DefaultMap, Terrain};
use shared::IslandId;
use std::collections::BTreeMap;

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

/// Visibility cases with equally close intermediate cells (Protocol 3.2).
pub const MIDPOINT_TIE_CASES: &[VisibilityCase] = &[
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

/// Reference visibility cases from the six corners of a hexagonal map.
pub const CORNER_CASES: &[VisibilityCase] = &[VisibilityCase {
    name: "corner_range",
    radius: 2,
    terrain: &[],
    vectors: &[
        VisibilityVector {
            from: Cell { q: 2, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 2, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 2, r: -2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 2, r: -2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: -2, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: -2, r: 0 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: -2, r: 2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: -2, r: 2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 2,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: 2 },
            to: Cell { q: 0, r: 0 },
            vision_range: 1,
            expected: false,
        },
        // Opposite corners across the map's full diameter.
        VisibilityVector {
            from: Cell { q: 2, r: 0 },
            to: Cell { q: -2, r: 0 },
            vision_range: 4,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 2, r: 0 },
            to: Cell { q: -2, r: 0 },
            vision_range: 3,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 2, r: -2 },
            to: Cell { q: -2, r: 2 },
            vision_range: 4,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 2, r: -2 },
            to: Cell { q: -2, r: 2 },
            vision_range: 3,
            expected: false,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -2 },
            to: Cell { q: 0, r: 2 },
            vision_range: 4,
            expected: true,
        },
        VisibilityVector {
            from: Cell { q: 0, r: -2 },
            to: Cell { q: 0, r: 2 },
            vision_range: 3,
            expected: false,
        },
    ],
}];

/// Runs the public visibility vectors against a supplied `LoS` implementation.
///
/// The supplied function is tested in both directions for every vector.
///
/// # Panics
///
/// Panics if a reference map is invalid or a visibility result differs
/// from the expected result.
pub fn test_visible<F>(mut visible: F)
where
    F: FnMut(&DefaultMap, Cell, Cell, u32) -> bool,
{
    let visibility_cases = MIDPOINT_TIE_CASES
        .iter()
        .chain(RANGE_CASES.iter())
        .chain(OBSTACLE_CASES.iter())
        .chain(ISLAND_CASES.iter())
        .chain(SURROUNDED_CASES.iter())
        .chain(CORNER_CASES.iter());

    for case in visibility_cases {
        // Build a complete clear-water map.
        let mut cells: BTreeMap<Cell, Terrain> = Cell { q: 0, r: 0 }
            .disc(case.radius)
            .map(|cell| (cell, Terrain::Clear))
            .collect();

        // Apply the terrain specified by the test vector.
        for &(cell, terrain) in case.terrain {
            *cells.get_mut(&cell).expect("Vector terrain must be inside the map") = terrain;
        }

        let map = DefaultMap::try_new(case.radius, cells).expect("Vector map must be valid");

        // Verify each expected result in both directions.
        for vector in case.vectors {
            assert_eq!(
                visible(&map, vector.from, vector.to, vector.vision_range),
                vector.expected,
                "Forward visibility failed in case: {}",
                case.name
            );

            assert_eq!(
                visible(&map, vector.to, vector.from, vector.vision_range),
                vector.expected,
                "Reverse visibility failed in case: {}",
                case.name
            );
        }
    }
}
