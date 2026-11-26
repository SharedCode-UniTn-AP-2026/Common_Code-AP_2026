use ap_2026_common_code_geometry::{Cell, DefaultMap, Map, MapError, Terrain};
use shared::IslandId;

// Builds the complete list of clear-water cells for a map.
fn clear_cells(radius: u32) -> Vec<(Cell, Terrain)> {
    (Cell { q: 0, r: 0 })
        .disc(radius)
        .map(|cell| (cell, Terrain::Clear))
        .collect()
}

// Builds a valid clear-water map.
fn clear_map(radius: u32) -> DefaultMap {
    DefaultMap::try_new(radius, clear_cells(radius)).expect("Clear map must be valid")
}

// Builds the radius-one mixed-terrain map used by the tests.
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
fn test_contains_multiple_radii() {
    for radius in [0, 1, 2, 3, 5] {
        let map = clear_map(radius);
        let r = i32::try_from(radius).expect("Small test radius fits in i32");

        assert!(map.contains(Cell { q: 0, r: 0 }));
        for corner in map.corners() {
            assert!(
                map.contains(corner),
                "Corner {corner:?} should be inside radius {radius}"
            );
        }

        // Test both signs of the q and r bounds.
        for cell in [
            Cell { q: r + 1, r: 0 },
            Cell { q: -r - 1, r: 0 },
            Cell { q: 0, r: r + 1 },
            Cell { q: 0, r: -r - 1 },
        ] {
            assert!(!map.contains(cell), "Cell {cell:?} should be outside radius {radius}");
        }

        if radius > 0 {
            // q and r individually fit, but s = -q - r is out of bounds.
            assert!(!map.contains(Cell { q: r, r }));
            assert!(!map.contains(Cell { q: -r, r: -r }));
        }
    }
}

#[test]
fn test_corners_multiple_radii() {
    // Compare with literal coordinates, independently of Direction::offset().
    for radius in [0, 1, 2, 3, 5, 10] {
        let r = i32::try_from(radius).expect("Small test radius fits in i32");
        let expected = [
            Cell { q: r, r: 0 },
            Cell { q: r, r: -r },
            Cell { q: 0, r: -r },
            Cell { q: -r, r: 0 },
            Cell { q: -r, r },
            Cell { q: 0, r },
        ];

        assert_eq!(
            clear_map(radius).corners(),
            expected,
            "Incorrect corners for radius {radius}"
        );
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

// Constructor error checks: errors are distinct, and their precedence matters.
#[test]
fn test_try_new_too_few_cells() {
    let mut cells = clear_cells(1);
    cells.pop();
    assert_eq!(DefaultMap::try_new(1, cells), Err(MapError::ErrNumCell));
}

#[test]
fn test_try_new_too_many_cells() {
    let mut cells = clear_cells(1);
    cells.push((Cell { q: 2, r: 0 }, Terrain::Clear));
    assert_eq!(DefaultMap::try_new(1, cells), Err(MapError::ErrNumCell));
}

#[test]
fn test_try_new_duplicate_cells() {
    let mut cells = clear_cells(1);
    cells[0] = cells[1]; // Exactly seven entries, but one duplicate and one missing cell.
    assert_eq!(DefaultMap::try_new(1, cells), Err(MapError::ErrCellDup));
}

#[test]
fn test_try_new_cell_outside_map() {
    // Exercise the q, r, and s boundary checks with the correct cell count.
    for outside in [Cell { q: 2, r: 0 }, Cell { q: 0, r: -2 }, Cell { q: 1, r: 1 }] {
        let mut cells = clear_cells(1);
        cells[0].0 = outside;
        assert_eq!(DefaultMap::try_new(1, cells), Err(MapError::ErrCellOut));
    }
}

#[test]
fn test_try_new_invalid_radius() {
    assert_eq!(
        DefaultMap::try_new(u32::MAX, Vec::<(Cell, Terrain)>::new()),
        Err(MapError::ErrRadius)
    );
}

#[test]
fn test_try_new_unordered_cells() {
    // Complete input sorted by (q, r), supplied in reverse order on purpose.
    let entries = [
        (Cell { q: -1, r: 0 }, Terrain::Rock),
        (Cell { q: -1, r: 1 }, Terrain::Cliff),
        (Cell { q: 0, r: -1 }, Terrain::Island(IslandId(1))),
        (Cell { q: 0, r: 0 }, Terrain::Clear),
        (Cell { q: 0, r: 1 }, Terrain::Rock),
        (Cell { q: 1, r: -1 }, Terrain::Clear),
        (Cell { q: 1, r: 0 }, Terrain::Island(IslandId(2))),
    ];
    let map = DefaultMap::try_new(1, entries.iter().copied().rev()).expect("Complete unordered map must be valid");

    for &(cell, terrain) in &entries {
        assert_eq!(map.terrain(cell), Some(terrain), "Incorrect terrain for {cell:?}");
    }
    assert_eq!(
        map.water_cells().collect::<Vec<_>>(),
        vec![
            Cell { q: -1, r: 0 },
            Cell { q: 0, r: 0 },
            Cell { q: 0, r: 1 },
            Cell { q: 1, r: -1 },
        ]
    );
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
