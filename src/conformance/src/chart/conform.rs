use super::Chart;
use geometry::Cell;
use rand::RngExt;
use shared::Params;
use std::fmt::Debug;

/// Deterministic generation, given a [`Seed`], [`Chart::generate`] should generate (or fail) the same way always.
/// By default it runs 5 times with different seeds
///
/// # Panics
///
/// Panics if the generation is not deterministic.
/// Eg. given the same seed two different archipelagos are created or different errors are produced
///
/// # Examples
/// ```
/// # use ap_2026_common_code_conformance::chart::{deterministic_generation, RefChart};
/// deterministic_generation::<RefChart>();
/// ```
pub fn deterministic_generation<C: Chart + PartialEq + Debug>() {
    for i in 0..=5 {
        let mut rng = rand::rng();

        let seed: u64 = rng.random();
        let mut params = Params::default();

        params = params.with_cliff_count(1);

        dbg!((i, seed));

        let gen_1 = C::generate(seed, &params);
        let gen_2 = C::generate(seed, &params);

        assert_eq!(gen_1, gen_2);
    }
}

/* pub fn placed_n_rocks<C: Chart + PartialEq + Debug>() {
    let cent = Cell { q: 0, r: 0 };
    let mut params = Params::default();

    params.with_cliff_count(1);

    let seed: u64 = rng.random();
    let ap = C::generate(seed, &params);

    assert!(C);
    let map = C::map(&self);

    let cliff_c = cent.disc(params.archipelago_radius).fold(0, |acc, c| {
        if let Some(t) = map.terrain(c) {
            match t {
                Terrain::Cliff => acc + 1,
                _ => acc + 0,
            }
        } else {
            acc + 0
        }
    });

    let rock_c = cent.disc(params.archipelago_radius).fold(0, |acc, c| {
        if let Some(t) = map.terrain(c) {
            match t {
                Terrain::Rock => acc + 1,
                _ => acc + 0,
            }
        } else {
            acc + 0
        }
    });

    println!(
        "c: {}, r: {}, pc: {}, pr: {}",
        cliff_c, rock_c, params.cliff_count, params.rock_count
    );
} */
