#![allow(clippy::pedantic)] // no need for pedantic clippy here

use std::collections::HashSet;

use super::*; // brings everything from clues to tests
use geometry::{DefaultMap, Map, Terrain};

// a small helper to create cells inline
fn cell(q: i32, r: i32) -> Cell {
    Cell { q, r }
}

// helper for map generation of arbitrary size and with all clear terrain
pub fn create_empty_map(radius: u32) -> DefaultMap {
    let cells = Cell { q: 0, r: 0 }.disc(radius).map(|cell| (cell, Terrain::Clear));

    DefaultMap::try_new(radius, cells).expect("valid radius for clear_map")
}

fn true_cells(map: &impl Map, clue: Clue) -> HashSet<Cell> {
    Cell { q: 0, r: 0 }
        .disc(map.radius())
        .filter(|&c| map.contains(c) && clue.holds(c))
        .collect()
}

mod common_tests {
    use super::*;

    #[test]
    fn display_variants() {
        let c1 = Cell { q: 1, r: -2 };
        let c2 = Cell { q: 17, r: 91 };
        let cases = [
            (
                Clue::Within { centre: c1, radius: 3 },
                "The treasure is within 3 units of (1,-2,1) cell",
            ),
            (
                Clue::Beyond { centre: c1, radius: 3 },
                "The treasure is beyond 3 units from the (1,-2,1) cell",
            ),
            (
                Clue::HalfPlane {
                    axis: Axis::Q,
                    bound: Bound::AtLeast(4),
                },
                "The treasure is at least 4 on the Q axis",
            ),
            (
                Clue::HalfPlane {
                    axis: Axis::S,
                    bound: Bound::AtMost(-2),
                },
                "The treasure is at most -2 on the S axis",
            ),
            (
                Clue::Nearer { a: c1, b: c2 },
                "The treasure is nearer to (1, -2, 1) than to (17, 91, -108)",
            ),
        ];

        for (clue, expected) in cases {
            assert_eq!(clue.to_string(), expected);
        }
    }

    // The most important test: a weakened clue's solutions are a superset of the original's
    #[test]
    fn weakened_clue_accepts_everything_the_original_accepted() {
        let map = create_empty_map(90);

        let clues = [
            Clue::Within {
                centre: cell(-32, -24),
                radius: 3,
            },
            Clue::Beyond {
                centre: cell(42, -43),
                radius: 4,
            },
            Clue::HalfPlane {
                axis: Axis::S,
                bound: Bound::AtLeast(-15),
            },
            Clue::HalfPlane {
                axis: Axis::Q,
                bound: Bound::AtMost(20),
            },
            Clue::Nearer {
                a: cell(-13, 15),
                b: cell(5, -10),
            },
        ];

        for clue in clues {
            let original = true_cells(&map, clue);

            // weakens the clues multiple times
            for times in [0, 1, 2, 3, 5] {
                let Ok(weaker) = clue.weaken(map.radius(), times) else {
                    assert!(matches!(clue, Clue::Nearer { .. }), "{clue:?}, times {times}");
                    continue;
                };
                let weaker_cells = true_cells(&map, weaker);
                let lost: Vec<_> = original.difference(&weaker_cells).collect();

                assert!(
                    lost.is_empty(),
                    "{clue:?} -> {weaker:?}, times {times}, lost cells {lost:?}"
                );
            }
        }
    }

    #[test]
    fn weaken_and_is_weaker_than_agree() {
        let map = create_empty_map(30);

        let clues = [
            Clue::Within {
                centre: cell(1, -2),
                radius: 3,
            },
            Clue::Beyond {
                centre: cell(0, 0),
                radius: 10,
            },
            Clue::HalfPlane {
                axis: Axis::S,
                bound: Bound::AtLeast(-2),
            },
            Clue::HalfPlane {
                axis: Axis::Q,
                bound: Bound::AtMost(3),
            },
            Clue::Nearer {
                a: cell(0, 0),
                b: cell(2, 1),
            },
            Clue::Nearer {
                a: cell(3, -4),
                b: cell(4, -3),
            }, // non-origin a
            Clue::Nearer {
                a: cell(0, 0),
                b: cell(-3, 2),
            }, // negative direction
        ];

        for clue in clues {
            for times in [2, 3, 5] {
                let weaker = clue
                    .weaken(map.radius(), times)
                    .unwrap_or_else(|e| panic!("{clue:?}, times {times}: {e}"));

                assert_eq!(weaker.is_weaker_than(clue), Ok(true));
                assert_eq!(clue.is_weaker_than(weaker), Ok(false));
            }
        }
    }
}

mod holds_tests {
    use super::*;

    #[test]
    fn within_holds_up_to_radius() {
        let clue = Clue::Within {
            centre: cell(0, 0),
            radius: 2,
        };

        assert!(clue.holds(cell(0, 0)));
        assert!(clue.holds(cell(2, 0))); // exactly on the boundary
        assert!(!clue.holds(cell(3, 0))); // outside within
    }

    #[test]
    fn beyond_holds_strictly_past_radius() {
        let clue = Clue::Beyond {
            centre: cell(0, 0),
            radius: 2,
        };

        assert!(clue.holds(cell(3, 0)));
        assert!(!clue.holds(cell(2, 0))); // exactly on the boundary
        assert!(!clue.holds(cell(0, 0))); // inside beyond
    }

    #[test]
    fn half_plane_bounds_are_inclusive() {
        let c = cell(3, -1); // q = 3, r = -1, s = -2

        // tests the Q axis with AtLeast
        let at_least = |k| Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(k),
        };
        // tests the Q axis with AtMost
        let at_most = |k| Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtMost(k),
        };

        assert!(at_least(3).holds(c));
        assert!(!at_least(4).holds(c));
        assert!(at_most(4).holds(c));
        assert!(!at_most(2).holds(c));
    }

    #[test]
    fn nearer_holds_when_closer() {
        let clue = Clue::Nearer {
            a: cell(0, 0),
            b: cell(4, 0),
        };

        assert!(clue.holds(cell(1, 0)));
        assert!(clue.holds(cell(-1, -1))); // closer to a
        assert!(!clue.holds(cell(3, 0))); // closer to b
    }

    #[test]
    fn nearer_not_holds_on_equal_distances() {
        let clue = Clue::Nearer {
            a: cell(0, 0),
            b: cell(4, 0),
        };

        // (2,0) is at distance 2 from both a and b
        assert!(!clue.holds(cell(2, 0)));
    }
}

mod is_weaker_than_tests {
    use super::*;

    #[test]
    fn within_larger_radius_is_weaker() {
        let c = cell(0, 0);
        let weak = Clue::Within { centre: c, radius: 5 };
        let strong = Clue::Within { centre: c, radius: 2 };

        assert_eq!(weak.is_weaker_than(strong), Ok(true));
        assert_eq!(strong.is_weaker_than(weak), Ok(false));
    }

    #[test]
    fn beyond_smaller_radius_is_weaker() {
        let c = cell(0, 0);
        let weak = Clue::Beyond { centre: c, radius: 1 };
        let strong = Clue::Beyond { centre: c, radius: 4 };

        assert_eq!(weak.is_weaker_than(strong), Ok(true));
        assert_eq!(strong.is_weaker_than(weak), Ok(false));
    }

    #[test]
    fn half_plane_lower_at_least_bound_is_weaker() {
        let weak = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(1),
        };
        let strong = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(5),
        };

        assert_eq!(weak.is_weaker_than(strong), Ok(true));
        assert_eq!(strong.is_weaker_than(weak), Ok(false));
    }

    #[test]
    fn nearer_farther_b_is_weaker() {
        let a = cell(0, 0);
        let weak = Clue::Nearer { a, b: cell(3, 0) };
        let strong = Clue::Nearer { a, b: cell(1, 0) };

        assert_eq!(weak.is_weaker_than(strong), Ok(true));
        assert_eq!(strong.is_weaker_than(weak), Ok(false));
    }

    #[test]
    fn equal_nearer_clues_are_not_weaker() {
        let clue = Clue::Nearer {
            a: cell(0, 0),
            b: cell(2, 0),
        };

        assert_eq!(clue.is_weaker_than(clue), Ok(false));
    }

    // ---- Same type, incompatible ----

    #[test]
    fn within_with_different_centres_fails() {
        let x = Clue::Within {
            centre: cell(0, 0),
            radius: 3,
        };
        let y = Clue::Within {
            centre: cell(1, 0),
            radius: 3,
        };

        assert!(x.is_weaker_than(y).is_err());
    }

    #[test]
    fn beyond_with_different_centres_fails() {
        let x = Clue::Beyond {
            centre: cell(0, 0),
            radius: 3,
        };
        let y = Clue::Beyond {
            centre: cell(1, 0),
            radius: 3,
        };

        assert!(x.is_weaker_than(y).is_err());
    }

    #[test]
    fn half_plane_with_different_axes_fails() {
        let x = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(1),
        };
        let y = Clue::HalfPlane {
            axis: Axis::R,
            bound: Bound::AtLeast(1),
        };

        assert!(x.is_weaker_than(y).is_err());
    }

    #[test]
    fn half_plane_with_different_bound_fails() {
        let x = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(1),
        };
        let y = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtMost(1),
        };

        assert!(x.is_weaker_than(y).is_err());
    }

    #[test]
    fn nearer_with_different_a_fails() {
        let x = Clue::Nearer {
            a: cell(0, 0),
            b: cell(2, 0),
        };
        let y = Clue::Nearer {
            a: cell(1, 0),
            b: cell(2, 0),
        };

        assert!(x.is_weaker_than(y).is_err());
    }

    // ---- Different types ----

    #[test]
    fn different_clues_cannot_be_compared() {
        let within = Clue::Within {
            centre: cell(0, 0),
            radius: 3,
        };
        let beyond = Clue::Beyond {
            centre: cell(0, 0),
            radius: 3,
        };
        let half = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(0),
        };
        let nearer = Clue::Nearer {
            a: cell(0, 0),
            b: cell(1, 0),
        };

        assert!(within.is_weaker_than(beyond).is_err());
        assert!(beyond.is_weaker_than(half).is_err());
        assert!(half.is_weaker_than(nearer).is_err());
        assert!(nearer.is_weaker_than(within).is_err());
    }
}

mod weaken_tests {
    use super::*;

    #[test]
    fn within_is_weakened_as_expected() {
        let map = create_empty_map(100);

        let within = Clue::Within {
            centre: cell(5, 5),
            radius: 4,
        };

        let expected_radius = |t| Clue::Within {
            centre: cell(5, 5),
            radius: t,
        };

        assert_eq!(within.weaken(map.radius(), 2), Ok(expected_radius(6)));
    }

    #[test]
    fn within_with_times_that_overflows() {
        let map = create_empty_map(100);

        let within = Clue::Within {
            centre: cell(5, 5),
            radius: i32::MAX as u32,
        };

        let expected_radius = |t| Clue::Within {
            centre: cell(5, 5),
            radius: t,
        };

        assert_eq!(within.weaken(map.radius(), u32::MAX), Ok(expected_radius(u32::MAX)));
    }
    #[test]
    fn half_plane_fully_relaxed_covers_whole_map() {
        let map = create_empty_map(15);
        let total = true_cells(
            &map,
            Clue::Within {
                centre: cell(0, 0),
                radius: 15,
            },
        )
        .len();

        let at_least = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(100),
        };
        let at_most = Clue::HalfPlane {
            axis: Axis::S,
            bound: Bound::AtMost(-100),
        };

        let weak_at_least = at_least.weaken(map.radius(), u32::MAX).unwrap();
        let weak_at_most = at_most.weaken(map.radius(), u32::MAX).unwrap();

        assert_eq!(true_cells(&map, weak_at_least).len(), total);
        assert_eq!(true_cells(&map, weak_at_most).len(), total);
    }

    #[test]
    fn half_plane_relax_does_not_overflow() {
        let map = create_empty_map(15);

        let at_least = Clue::HalfPlane {
            axis: Axis::Q,
            bound: Bound::AtLeast(i32::MIN + 1),
        };
        let at_most = Clue::HalfPlane {
            axis: Axis::R,
            bound: Bound::AtMost(i32::MAX - 1),
        };

        // overshoots the i32 range by a lot: must clamp, not wrap or panic
        assert_eq!(
            at_least.weaken(map.radius(), u32::MAX),
            Ok(Clue::HalfPlane {
                axis: Axis::Q,
                bound: Bound::AtLeast(i32::MIN)
            })
        );
        assert_eq!(
            at_most.weaken(map.radius(), u32::MAX),
            Ok(Clue::HalfPlane {
                axis: Axis::R,
                bound: Bound::AtMost(i32::MAX)
            })
        );

        // lands exactly on the limit: exact result, no clamping needed
        assert_eq!(
            at_least.weaken(map.radius(), 1),
            Ok(Clue::HalfPlane {
                axis: Axis::Q,
                bound: Bound::AtLeast(i32::MIN)
            })
        );
        assert_eq!(
            at_most.weaken(map.radius(), 1),
            Ok(Clue::HalfPlane {
                axis: Axis::R,
                bound: Bound::AtMost(i32::MAX)
            })
        );
    }

    #[test]
    fn nearer_goes_outside_the_map() {
        let map = create_empty_map(100);
        let a = cell(0, 0);
        let b = cell(10, 0);
        let clue = Clue::Nearer { a, b };

        // times = 10 -> b' = (100, 0): exactly on the edge, still valid
        assert_eq!(clue.weaken(map.radius(), 10), Ok(Clue::Nearer { a, b: cell(100, 0) }));

        // times = 11 -> b' = (110, 0): one step past the edge
        assert_eq!(
            clue.weaken(map.radius(), 11),
            Err("Nearer: new cell is outside the map, reduce times param".into())
        );

        // huge times must be an error too, not an overflow or a wrap
        assert_eq!(
            clue.weaken(map.radius(), u32::MAX),
            Err("Nearer: coordinates overflow, reduce times param".into())
        );
    }

    #[test]
    fn nearer_goes_outside_the_map_a_offcenter() {
        let map = create_empty_map(100);
        let a = cell(-30, 10);
        let b = cell(-28, 9); // direction (2, -1)
        let clue = Clue::Nearer { a, b };

        // new_b = (-30 + 2t, 10 - t)
        // t = 65 -> (100, -55): q is exactly at the edge (s = -45), still valid
        assert_eq!(clue.weaken(map.radius(), 65), Ok(Clue::Nearer { a, b: cell(100, -55) }));

        // t = 66 -> (102, -56): outside
        assert_eq!(
            clue.weaken(map.radius(), 66),
            Err("Nearer: new cell is outside the map, reduce times param".into())
        );
    }

    #[test]
    fn nearer_same_a_and_b_is_not_weakened() {
        let map = create_empty_map(100);
        let clue = Clue::Nearer {
            a: cell(2, 3),
            b: cell(2, 3),
        };

        // no direction to scale along, so the clue comes back unchanged
        assert_eq!(clue.weaken(map.radius(), 5), Ok(clue));
        assert_eq!(clue.weaken(map.radius(), u32::MAX), Ok(clue));
    }

    #[test]
    fn nearer_with_small_times_is_not_weakened() {
        let map = create_empty_map(100);
        let clue = Clue::Nearer {
            a: cell(0, 0),
            b: cell(4, 0),
        };

        // times = 0 and 1 leave b where it is
        assert_eq!(clue.weaken(map.radius(), 0), Ok(clue));
        assert_eq!(clue.weaken(map.radius(), 1), Ok(clue));

        // times = 2 is the first value that actually moves b: (8, 0)
        assert_eq!(
            clue.weaken(map.radius(), 2),
            Ok(Clue::Nearer {
                a: cell(0, 0),
                b: cell(8, 0)
            })
        );
    }
}
