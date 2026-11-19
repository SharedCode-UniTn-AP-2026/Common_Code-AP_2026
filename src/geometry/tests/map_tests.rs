//! Regression tests adapted from the original pre-GitHub Map test suite.
//! The current API uses the `Map` trait and the concrete `DefaultMap` type.

use ap_2026_common_code_geometry::{Cell, DefaultMap, Direction, Map, Terrain};
use shared::IslandId;

/// Builds a valid hexagonal map with clear water in every cell.
fn clear_map(radius: u32) -> DefaultMap {
    let cells = Cell { q: 0, r: 0 }.disc(radius).map(|cell| (cell, Terrain::Clear));
    DefaultMap::try_new(radius, cells).expect("Clear map must be valid")
}

/// Builds the mixed-terrain radius-one map used by the original tests.
fn mixed_map() -> DefaultMap {
    DefaultMap::try_new(
        1,
        [
            (Cell { q: 0, r: 0 }, Terrain::Clear),
            (Cell { q: 1, r: 0 }, Terrain::Rock),
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 0, r: -1 }, Terrain::Cliff),
            (Cell { q: -1, r: 0 }, Terrain::Clear),
            (Cell { q: -1, r: 1 }, Terrain::Clear),
            (Cell { q: 0, r: 1 }, Terrain::Clear),
        ],
    )
    .expect("Mixed-terrain map must be valid")
}

#[test]
fn test_contains() {
    let map = clear_map(3);

    for cell in [
        Cell { q: 0, r: 0 },
        Cell { q: 3, r: 0 },
        Cell { q: 3, r: -3 },
        Cell { q: -3, r: 0 },
        Cell { q: 0, r: 3 },
    ] {
        assert!(map.contains(cell), "Cell {cell:?} should be inside");
    }

    for cell in [Cell { q: 4, r: 0 }, Cell { q: 2, r: 2 }, Cell { q: -4, r: 1 }] {
        assert!(!map.contains(cell), "Cell {cell:?} should be outside");
    }
}

#[test]
fn test_terrain() {
    let map = mixed_map();

    assert_eq!(map.terrain(Cell { q: 0, r: 0 }), Some(Terrain::Clear));
    assert_eq!(map.terrain(Cell { q: 1, r: 0 }), Some(Terrain::Rock));
    assert_eq!(map.terrain(Cell { q: 1, r: -1 }), Some(Terrain::Island(IslandId(1))));
    assert_eq!(map.terrain(Cell { q: 0, r: -1 }), Some(Terrain::Cliff));
    assert_eq!(map.terrain(Cell { q: 2, r: 0 }), None);
}

#[test]
fn test_is_land() {
    let map = mixed_map();

    assert!(map.is_land(Cell { q: 1, r: -1 }));
    assert!(map.is_land(Cell { q: 0, r: -1 }));
    assert!(!map.is_land(Cell { q: 0, r: 0 }));
    assert!(!map.is_land(Cell { q: 1, r: 0 }));
    assert!(!map.is_land(Cell { q: 2, r: 0 }));
}

#[test]
fn test_is_water() {
    let map = mixed_map();

    assert!(map.is_water(Cell { q: 0, r: 0 }));
    assert!(map.is_water(Cell { q: 1, r: 0 }));
    assert!(!map.is_water(Cell { q: 1, r: -1 }));
    assert!(!map.is_water(Cell { q: 0, r: -1 }));
    assert!(!map.is_water(Cell { q: 2, r: 0 }));
}

#[test]
fn test_corners() {
    let map = clear_map(3);
    let expected = [
        Cell { q: 3, r: 0 },
        Cell { q: 3, r: -3 },
        Cell { q: 0, r: -3 },
        Cell { q: -3, r: 0 },
        Cell { q: -3, r: 3 },
        Cell { q: 0, r: 3 },
    ];

    assert_eq!(map.corners(), expected);
    for corner in map.corners() {
        assert!(map.contains(corner));
    }
}

#[test]
fn test_radius() {
    for radius in [0, 1, 3, 5, 10] {
        assert_eq!(clear_map(radius).radius(), radius);
    }
}

#[test]
fn test_land_water_exclusive() {
    let map = mixed_map();

    for cell in (Cell { q: 0, r: 0 }).disc(1) {
        assert_ne!(
            map.is_land(cell),
            map.is_water(cell),
            "Cell {cell:?} must be either land or water, not both"
        );
    }

    let outside = Cell { q: 2, r: 0 };
    assert!(!map.is_land(outside));
    assert!(!map.is_water(outside));
}

#[test]
fn test_contains_multiple_radii() {
    for radius in [0, 1, 2, 3, 5] {
        let map = clear_map(radius);
        let r = i32::try_from(radius).expect("Small test radius fits in i32");

        assert!(map.contains(Cell { q: 0, r: 0 }));
        for corner in map.corners() {
            assert!(
                map.contains(corner),
                "Corner {corner:?} should be inside a radius-{radius} map"
            );
        }

        assert!(!map.contains(Cell { q: r + 1, r: 0 }));
        assert!(!map.contains(Cell { q: 0, r: -r - 1 }));
        if radius > 0 {
            // q and r are in range, but s = -q - r is not.
            assert!(!map.contains(Cell { q: r, r }));
        }
    }
}

#[test]
fn test_corners_multiple_radii() {
    let directions = [
        Direction::East,
        Direction::NorthEast,
        Direction::NorthWest,
        Direction::West,
        Direction::SouthWest,
        Direction::SouthEast,
    ];
    let origin = Cell { q: 0, r: 0 };

    for radius in [1, 2, 3, 5, 10] {
        let map = clear_map(radius);
        let corners = map.corners();
        let r = i32::try_from(radius).expect("Small test radius fits in i32");

        for (i, direction) in directions.iter().enumerate() {
            let (dq, dr) = direction.offset();
            let expected = Cell { q: dq * r, r: dr * r };

            assert_eq!(corners[i], expected);
            assert_eq!(origin.distance(corners[i]), radius);
        }

        for i in 0..6 {
            for j in (i + 1)..6 {
                assert_ne!(corners[i], corners[j]);
            }
        }
    }
}

#[test]
fn test_water_cells() {
    let water: Vec<Cell> = mixed_map().water_cells().collect();
    let expected = vec![
        Cell { q: -1, r: 0 },
        Cell { q: -1, r: 1 },
        Cell { q: 0, r: 0 },
        Cell { q: 0, r: 1 },
        Cell { q: 1, r: 0 },
    ];
    assert_eq!(water, expected);
}

#[test]
fn test_clear_cells() {
    let clear: Vec<Cell> = mixed_map().clear_cells().collect();
    let expected = vec![
        Cell { q: -1, r: 0 },
        Cell { q: -1, r: 1 },
        Cell { q: 0, r: 0 },
        Cell { q: 0, r: 1 },
    ];
    assert_eq!(clear, expected);
}
