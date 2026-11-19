use ap_2026_common_code_geometry::{Cell, DefaultMap, Terrain, visible};
use shared::IslandId;
use std::collections::BTreeMap;

/// Builds a complete clear-water map with the specified terrain replacements.
///
/// # Panics
///
/// Panics if the radius is too large for `Cell::disc` or `DefaultMap::try_new`, a
/// replacement lies outside the map, or map construction fails validation.
fn test_map(radius: u32, replacements: &[(Cell, Terrain)]) -> DefaultMap {
    let mut terrain: BTreeMap<_, _> = Cell { q: 0, r: 0 }
        .disc(radius)
        .map(|cell| (cell, Terrain::Clear))
        .collect();

    for &(cell, replacement) in replacements {
        *terrain
            .get_mut(&cell)
            .expect("Replacement cell must be inside the test map") = replacement;
    }

    DefaultMap::try_new(radius, terrain).expect("Test map must be complete and valid")
}

// Checks that a cell can see itself even with zero vision range and land terrain.
#[test]
fn test_visible_same_cell() {
    let cell = Cell { q: 0, r: 0 };
    let map = test_map(0, &[(cell, Terrain::Island(IslandId(1)))]);

    assert!(visible(&map, cell, cell, 0));
}

// Checks that cells beyond the maximum vision range are not visible.
#[test]
fn test_visible_out_of_range() {
    let from = Cell { q: 0, r: 0 };
    let to = Cell { q: 2, r: 0 };
    let map = test_map(2, &[]);

    // The distance is 2, but the vision range is only 1.
    assert_eq!(from.distance(to), 2);
    assert!(!visible(&map, from, to, 1));
}

// Checks that two cells are visible when the line between them contains only clear water.
#[test]
fn test_visible_clear_path() {
    let from = Cell { q: -1, r: 0 };
    let to = Cell { q: 1, r: 0 };

    // Complete radius-1 map with only clear water.
    let map = test_map(1, &[]);

    assert_eq!(from.distance(to), 2);

    // Both cells are exactly within the vision range.
    assert!(visible(&map, from, to, 2));
}

// Checks that an island directly between two cells blocks line of sight.
#[test]
fn test_visible_land_blocked() {
    let from = Cell { q: -1, r: 0 };
    let to = Cell { q: 1, r: 0 };
    let map = test_map(1, &[(Cell { q: 0, r: 0 }, Terrain::Island(IslandId(1)))]);

    assert_eq!(from.distance(to), 2);

    // The island in the middle must block visibility.
    assert!(!visible(&map, from, to, 2));
}

// Checks that a cliff directly between two cells blocks line of sight.
#[test]
fn test_visible_cliff_blocked() {
    let from = Cell { q: -1, r: 0 };
    let to = Cell { q: 1, r: 0 };
    let map = test_map(1, &[(Cell { q: 0, r: 0 }, Terrain::Cliff)]);

    assert_eq!(from.distance(to), 2);

    // The cliff in the middle must block visibility.
    assert!(!visible(&map, from, to, 2));
}

// Checks that a rock between two cells does not block line of sight.
#[test]
fn test_visible_rock() {
    let from = Cell { q: -1, r: 0 };
    let to = Cell { q: 1, r: 0 };
    let map = test_map(1, &[(Cell { q: 0, r: 0 }, Terrain::Rock)]);

    assert_eq!(from.distance(to), 2);

    // Rock is water and must not block visibility.
    assert!(visible(&map, from, to, 2));
}

// Checks that a line passing between land and water remains visible.
#[test]
fn test_visible_boundary_land_water() {
    let from = Cell { q: 0, r: 0 };
    let to = Cell { q: 2, r: -1 };

    // The line passes exactly between these two cells.
    let map = test_map(
        2,
        &[
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 1, r: 0 }, Terrain::Clear),
        ],
    );

    assert_eq!(from.distance(to), 2);

    // One candidate is water, so visibility must not be blocked.
    assert!(visible(&map, from, to, 2));
}

// Checks that a line passing between two land cells is blocked.
#[test]
fn test_visible_boundary_land_land() {
    let from = Cell { q: 0, r: 0 };
    let to = Cell { q: 2, r: -1 };

    // Both cells sharing the boundary are land.
    let map = test_map(
        2,
        &[
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 1, r: 0 }, Terrain::Cliff),
        ],
    );

    assert_eq!(from.distance(to), 2);

    // Both nearest candidates are land, so visibility is blocked.
    assert!(!visible(&map, from, to, 2));
}

// Checks that reversing the line of sight always gives the same visibility result.
#[test]
fn test_visible_symmetry() {
    let a = Cell { q: 0, r: 0 };
    let b = Cell { q: 2, r: -1 };

    // First case: one land cell and one water cell.
    let map = test_map(2, &[(Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1)))]);

    // Visibility must work in both directions.
    assert!(visible(&map, a, b, 2));
    assert!(visible(&map, b, a, 2));

    // Second case: both intermediate candidates are land.
    let map = test_map(
        2,
        &[
            (Cell { q: 1, r: -1 }, Terrain::Island(IslandId(1))),
            (Cell { q: 1, r: 0 }, Terrain::Cliff),
        ],
    );

    // Visibility must be blocked in both directions.
    assert!(!visible(&map, a, b, 2));
    assert!(!visible(&map, b, a, 2));
}

// Checks that land at the starting and ending cells does not block line of sight.
#[test]
fn test_visible_endpoints() {
    let from = Cell { q: 0, r: 0 };
    let to = Cell { q: 2, r: 0 };

    // Both endpoints are land.
    let map = test_map(2, &[(from, Terrain::Cliff), (to, Terrain::Island(IslandId(1)))]);

    assert_eq!(from.distance(to), 2);

    // Land endpoints must not block visibility.
    assert!(visible(&map, from, to, 2));
    assert!(visible(&map, to, from, 2));
}

// Checks that visibility includes cells exactly at the maximum vision range.
#[test]
fn test_visible_range_boundary() {
    let from = Cell { q: 0, r: 0 };
    let to = Cell { q: 2, r: 0 };
    let map = test_map(2, &[]);

    assert_eq!(from.distance(to), 2);

    // Distance exceeds the vision range.
    assert!(!visible(&map, from, to, 1));

    // Distance is exactly equal to the vision range.
    assert!(visible(&map, from, to, 2));

    // Distance is smaller than the vision range.
    assert!(visible(&map, from, to, 3));
}

// Checks that every interior sample is examined along a longer line of sight.
#[test]
fn test_visible_multiple_samples() {
    let from = Cell { q: -2, r: 0 };
    let to = Cell { q: 2, r: 0 };
    let map = test_map(2, &[]);

    assert_eq!(from.distance(to), 4);

    // All interior samples are clear.
    assert!(visible(&map, from, to, 4));

    // Place a cliff at the last interior sample.
    let map = test_map(2, &[(Cell { q: 1, r: 0 }, Terrain::Cliff)]);

    // Visibility must now be blocked.
    assert!(!visible(&map, from, to, 4));
}
