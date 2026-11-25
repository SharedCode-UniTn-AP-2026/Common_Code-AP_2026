use super::{Chart, RefChart};
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
        let params = Params::default();

        dbg!((i, seed));

        let gen_1 = C::generate(seed, &params);
        let gen_2 = C::generate(seed, &params);

        assert_eq!(gen_1, gen_2);
    }
}
