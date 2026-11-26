use super::{Chart, GenErrItem, GenError};
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

/// Fails step 2 of generate, because `archipelago_radius` <= `vision_range`
///
/// # Panics
///
/// Panics if step 2 does not respect `archipelago_radius` <= `vision_range`
///
/// # Example
/// ```
/// # use ap_2026_common_code_conformance::chart::{generate_fail_params, RefChart};
/// generate_fail_params::<RefChart>();
/// ```
pub fn generate_fail_params<C: Chart + PartialEq + Debug>() {
    #[derive(Debug)]
    struct TestCase {
        #[allow(dead_code)]
        info: &'static str,
        archipelago_radius: u32,
        vision_range: u32,
        should_fail: bool,
    }

    let mut rng = rand::rng();
    let seed: u64 = rng.random();

    let cases = [
        TestCase {
            info: "`archipelago_radius` == `vision_range`",
            archipelago_radius: 10,
            vision_range: 10,
            should_fail: true,
        },
        TestCase {
            info: "`archipelago_radius` < `vision_range`",
            archipelago_radius: 9,
            vision_range: 10,
            should_fail: true,
        },
        TestCase {
            info: "`archipelago_radius` > `vision_range`",
            archipelago_radius: 10,
            vision_range: 9,
            should_fail: false,
        },
    ];

    for tc in cases {
        let params = Params::default()
            .with_archipelago_radius(tc.archipelago_radius)
            .with_vision_range(tc.vision_range);

        dbg!(&tc);
        let gen_1 = C::generate(seed, &params);

        if tc.should_fail {
            assert_eq!(gen_1, Err(GenError::VisionRangeTooHigh));
        } else {
            assert_ne!(gen_1, Err(GenError::VisionRangeTooHigh));
        }
    }
}

/// Fails step 3 of generate, because archipelago cannot place `island_count`
///
/// # Panics
///
/// Panics if step 3 does not pass test
///
/// # Example
/// ```
/// # use ap_2026_common_code_conformance::chart::{generate_fail_island, RefChart};
/// generate_fail_island::<RefChart>();
/// ```
pub fn generate_fail_island<C: Chart + PartialEq + Debug>() {
    #[derive(Debug)]
    struct TestCase {
        #[allow(dead_code)]
        info: &'static str,
        archipelago_radius: u32,
        island_count: u8,
        should_fail: bool,
    }

    let mut rng = rand::rng();
    let seed: u64 = rng.random();

    let cases = [
        TestCase {
            info: "0 Islands, will never fail",
            archipelago_radius: 12,
            island_count: 0,
            should_fail: false,
        },
        TestCase {
            info: "1 Island, will never fail on a big enough archipelago",
            archipelago_radius: 12,
            island_count: 0,
            should_fail: false,
        },
        TestCase {
            info: "Will fail as archipelago is not big enough",
            archipelago_radius: 1,
            island_count: 2,
            should_fail: true,
        },
        TestCase {
            info: "Will fail, no good candidate cells",
            archipelago_radius: 1,
            island_count: 1,
            should_fail: true,
        },
    ];

    for tc in cases {
        let params = Params::default()
            .with_archipelago_radius(tc.archipelago_radius)
            .with_island_count(tc.island_count)
            .with_vision_range(tc.archipelago_radius - 1);

        dbg!(&tc);
        let gen_1 = C::generate(seed, &params);

        if tc.should_fail {
            assert_eq!(
                gen_1,
                Err(GenError::CantPlace {
                    item: GenErrItem::Island
                })
            );
        } else {
            assert_ne!(
                gen_1,
                Err(GenError::CantPlace {
                    item: GenErrItem::Island
                })
            );
        }
    }
}
