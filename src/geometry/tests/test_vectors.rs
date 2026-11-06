use ap_2026_common_code_geometry::{
    Cell, Map, Terrain,
    test_vectors::{ISLAND_CASES, OBSTACLE_CASES, PROTOCOL_3_2_CASES, RANGE_CASES, SURROUNDED_CASES},
    visible,
};
use std::collections::BTreeMap;

#[test]
fn test_protocol_3_2_vectors() {
    for case in PROTOCOL_3_2_CASES
        .iter()
        .chain(RANGE_CASES.iter())
        .chain(OBSTACLE_CASES.iter())
        .chain(ISLAND_CASES.iter())
        .chain(SURROUNDED_CASES.iter())
    {
        // Build a complete clear-water map.
        let mut cells: BTreeMap<Cell, Terrain> = Cell { q: 0, r: 0 }
            .disc(case.radius)
            .map(|cell| (cell, Terrain::Clear))
            .collect();

        // Apply the terrain specified by the test vector.
        for &(cell, terrain) in case.terrain {
            *cells.get_mut(&cell).expect("Vector terrain must be inside the map") = terrain;
        }

        let map = Map::try_new(case.radius, cells).expect("Vector map must be valid");

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
