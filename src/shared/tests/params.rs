use ap_2026_common_code_shared::Params;

#[test]
fn test_default_values() {
    let params = Params::default();

    assert_eq!(params.archipelago_radius, 12);
    assert_eq!(params.island_count, 8);
    assert_eq!(params.cliff_count, 6);
    assert_eq!(params.rock_count, 12);
    assert_eq!(params.vision_range, 6);
    assert_eq!(params.bombard_range, 2);
    assert_eq!(params.bombard_damage, 1);
    assert_eq!(params.hull_strength, 3);
    assert_eq!(params.round_cap, 200);
    assert_eq!(params.round_deadline_ms, 500);

    assert!(params.archipelago_radius > params.vision_range);
    assert!(params.bombard_range <= params.vision_range);
}

#[test]
fn test_with_methods() {
    let params = Params::default()
        .with_archipelago_radius(20)
        .with_island_count(9)
        .with_cliff_count(7)
        .with_rock_count(15)
        .with_vision_range(8)
        .with_bombard_range(3)
        .with_bombard_damage(2)
        .with_hull_strength(5)
        .with_round_cap(250)
        .with_round_deadline_ms(600);

    assert_eq!(params.archipelago_radius, 20);
    assert_eq!(params.island_count, 9);
    assert_eq!(params.cliff_count, 7);
    assert_eq!(params.rock_count, 15);
    assert_eq!(params.vision_range, 8);
    assert_eq!(params.bombard_range, 3);
    assert_eq!(params.bombard_damage, 2);
    assert_eq!(params.hull_strength, 5);
    assert_eq!(params.round_cap, 250);
    assert_eq!(params.round_deadline_ms, 600);
}

#[test]
fn test_rock_damage() {
    assert_eq!(Params::default().rock_damage(), 2);

    assert_eq!(Params::default().with_hull_strength(4).rock_damage(), 2);

    assert_eq!(Params::default().with_hull_strength(5).rock_damage(), 3);
}
