use ap_2026_common_code_geometry::{Cell, DefaultMap, Terrain, visible};
use proptest::prelude::*;
use shared::IslandId;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn test_visibility_symmetry_generated_maps(
        radius in 1u32..=3,
        terrain_codes in prop::collection::vec(0u8..4, 37),
        from_index in any::<usize>(),
        to_index in any::<usize>(),
        vision_range in 0u32..=6,
    ) {
        // Generate every cell of a valid hexagonal map.
        let cells: Vec<Cell> = Cell { q: 0, r: 0 }
            .disc(radius)
            .collect();

        // Assign generated terrain types to the cells.
        let terrain = cells.iter().enumerate().map(|(i, &cell)| {
            let terrain_type = match terrain_codes[i] {
                0 => Terrain::Clear,
                1 => Terrain::Rock,
                2 => Terrain::Cliff,
                _ => Terrain::Island(IslandId(1)),
            };

            (cell, terrain_type)
        });

        let map = DefaultMap::try_new(radius, terrain)
            .expect("Generated map must be valid");

        // Select two cells inside the map.
        let from = cells[from_index % cells.len()];
        let to = cells[to_index % cells.len()];

        // Visibility must be symmetric.
        let forward = visible(&map, from, to, vision_range);
        let backward = visible(&map, to, from, vision_range);

        prop_assert_eq!(forward, backward);
    }
}
