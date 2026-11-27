use super::Chart;
use geometry::{Cell, Map, Terrain};
use rand::RngExt;
use shared::Params;

/// Deterministic generation, given a [`Seed`], two [`Chart::generate`] should succeed (or fail) the same way always.
/// If a generation succeeds, the two generated [`Map`] are compared
/// Runs 5 times
///
/// # Panics
///
/// Panics if two implementations fail in different ways at different steps or if the generated [`Maps`] differ
///
/// # Examples
/// ```
/// use ap_2026_common_code_conformance::chart::{generate, RefChart};
/// generate::<RefChart, RefChart>();
/// ```
pub fn generate<C1: Chart, C2: Chart>() {
    for i in 0..=5 {
        let mut rng = rand::rng();

        let seed: u64 = rng.random();
        let params = Params::default();

        dbg!((i, seed));

        let gen_1 = C1::generate(seed, &params);
        let gen_2 = C2::generate(seed, &params);

        if gen_1.is_ok() && gen_2.is_ok() {
            let ok_1 = gen_1.ok().unwrap();
            let ok_2 = gen_2.ok().unwrap();

            let map_1 = ok_1.map();
            let map_2 = ok_2.map();

            assert_eq!(map_1.radius(), map_2.radius());

            assert!(Cell { q: 0, r: 0 }.disc(map_1.radius()).all(|cell| {
                if !map_1.contains(cell) || !map_2.contains(cell) {
                    false
                } else {
                    let some_1 = map_1.terrain(cell).is_some();
                    let some_2 = map_2.terrain(cell).is_some();

                    assert_eq!(some_1, some_2);

                    let t_1 = map_1.terrain(cell).unwrap();
                    let t_2 = map_2.terrain(cell).unwrap();

                    match t_1 {
                        Terrain::Island(_) => matches!(t_2, Terrain::Island(_)),
                        _ => t_1 == t_2,
                    }
                }
            }));
        } else {
            assert_eq!(gen_1.is_err(), gen_2.is_err());
            let err_1 = gen_1.err();
            let err_2 = gen_2.err();
            assert_eq!(err_1, err_2);
        }
    }
}

/// Check for validate actions
pub fn _validate<C1: Chart, C2: Chart>() {
    todo!()
}

/// Check for apply
pub fn _apply<C1: Chart, C2: Chart>() {
    todo!()
}

/// Check if two [`Chart`] advance in the same way
pub fn _advance_round<C1: Chart, C2: Chart>() {
    todo!()
}

/// Check if two [`Chart`] provide the same view to a [`Ship`]
pub fn _view_for<C1: Chart, C2: Chart>() {}
