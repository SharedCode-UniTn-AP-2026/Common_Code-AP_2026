use crate::chart::{Chart, GenErrItem, GenError, RefChart};
use rand::RngExt;
use shared::Params;

#[test]
pub fn generate_fail_params() {
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
        let gen_1 = RefChart::generate(seed, &params);

        if tc.should_fail {
            assert!(gen_1.is_err());
            assert_eq!(gen_1.err().unwrap(), GenError::VisionRangeTooHigh);
        } else if gen_1.is_err() {
            assert_ne!(gen_1.err().unwrap(), GenError::VisionRangeTooHigh);
        } else {
            assert!(gen_1.is_ok());
        }
    }
}

#[test]
pub fn generate_fail_island() {
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
        let gen_1 = RefChart::generate(seed, &params);

        if tc.should_fail {
            assert!(gen_1.is_err());

            assert_eq!(
                gen_1.err().unwrap(),
                GenError::CantPlace {
                    item: GenErrItem::Island
                }
            );
        } else if gen_1.is_err() {
            assert_ne!(gen_1.err().unwrap(), GenError::VisionRangeTooHigh);
        } else {
            assert!(gen_1.is_ok());
        }
    }
}
