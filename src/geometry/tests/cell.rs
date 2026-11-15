use ap_2026_common_code_geometry::{Cell, Direction};
use proptest::prelude::*;
use std::collections::HashSet;

/// The six directions in discriminant order, from `East` (0) to `SouthEast` (5),
/// as in the protocol table.
const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::East,
    Direction::NorthEast,
    Direction::NorthWest,
    Direction::West,
    Direction::SouthWest,
    Direction::SouthEast,
];

/// Generates any cell with `q` and `r` in `-1000..=1000`.
///
/// The range is small enough that no `Cell` method can overflow on it.
fn any_cell() -> impl Strategy<Value = Cell> {
    (-1000..=1000_i32, -1000..=1000_i32).prop_map(|(q, r)| Cell { q, r })
}

/// Radius bound checked first by `Map::try_new`, which returns
/// `MapError::ErrRadius` for any radius greater than it.
///
/// No `Map` has a radius greater than this, so no `Map` has a cell with `|q|`,
/// `|r|` or `|s|` greater than it. It is an upper bound, not the largest radius
/// of a real `Map`: `try_new` also requires the cell count to fit in `u32`,
/// which rejects this radius too and caps it far lower.
const MAX_MAP_RADIUS: i32 = i32::MAX / 2;

/// Largest radius accepted by `Cell::disc` and `Cell::ring`, which both panic
/// on any radius greater than `i32::MAX`.
const MAX_RADIUS: u32 = i32::MAX.cast_unsigned();

/// Generates any cell with `|q|`, `|r|` and `|s|` at most `MAX_MAP_RADIUS`.
///
/// This includes every cell of every `Map`.
fn map_cell() -> impl Strategy<Value = Cell> {
    (-MAX_MAP_RADIUS..=MAX_MAP_RADIUS).prop_flat_map(|q| {
        let lo = (-MAX_MAP_RADIUS).max(-MAX_MAP_RADIUS - q);
        let hi = MAX_MAP_RADIUS.min(MAX_MAP_RADIUS - q);
        (lo..=hi).prop_map(move |r| Cell { q, r })
    })
}

/// Returns the distance between `a` and `b` as `max(|dq|, |dr|, |ds|)`.
///
/// Used as an independent check of `Cell::distance`: it uses a different
/// formula and computes in `i64`, so the arithmetic itself cannot overflow.
///
/// # Panics
///
/// Panics if the distance exceeds `u32::MAX`, which can only happen when the
/// coordinates are close to the `i32` limits.
fn distance_max_formula(a: Cell, b: Cell) -> u32 {
    let dq = i64::from(a.q) - i64::from(b.q);
    let dr = i64::from(a.r) - i64::from(b.r);
    let ds = -dq - dr; // s_a - s_b = -(q_a + r_a) + (q_b + r_b) = -(dq + dr)

    let d = dq.abs().max(dr.abs()).max(ds.abs());
    u32::try_from(d).expect("distance does not fit in u32")
}

/// Returns whether `cells` is in strictly ascending order by `q`, then by `r`,
/// as the docs of `Cell::disc` and `Cell::ring` promise.
///
/// The order is strict, so it also returns `false` if a cell appears twice.
fn is_strictly_ascending(cells: &[Cell]) -> bool {
    cells.windows(2).all(|w| (w[0].q, w[0].r) < (w[1].q, w[1].r))
}

proptest! {
    // --- Distance ---
    //check the symmetry of the distance function
    #[test]
    fn test_distance_is_symmetric(a in any_cell(), b in any_cell()) {
        prop_assert_eq!(a.distance(b), b.distance(a));
    }

    //check the distance correctness using another equivalent formula
    #[test]
    fn test_distance_matches_max_formula(a in any_cell(), b in any_cell()) {
        prop_assert_eq!(a.distance(b), distance_max_formula(a, b));
    }

    // --- Neighbours ---
    //check that, if a != b, at least one neighbour of a is one step closer to b
    #[test]
    fn test_some_neighbour_is_closer(a in any_cell(), b in any_cell()) {
        prop_assume!(a != b);
        let d = a.distance(b);
        prop_assert!(a.neighbours().iter().any(|n| n.distance(b) == d - 1));
    }


    //check id all neighbours are at distance 1
    #[test]
    fn test_neighbours_at_distance_one(a in any_cell()) {
        for c in a.neighbours() {
            prop_assert_eq!(c.distance(a), 1);
        }
    }

    //check if the neighbours are all different
    #[test]
    fn test_neighbours_are_distinct(a in any_cell()) {
        let distinct: HashSet<Cell> = a.neighbours().into_iter().collect();
        prop_assert_eq!(distinct.len(), 6);
    }

    // --- Ring ---
    // Checks that a ring of radius d >= 1 has 6 * d cells, all at distance d,
    // in strictly ascending order. With no duplicates, these are all the cells
    // at distance d.
    #[test]
    fn test_ring(center in any_cell(), radius in 1..50_u32) {
        let ring: Vec<Cell> = center.ring(radius).collect();

        prop_assert_eq!(ring.len(), 6 * radius as usize);
        for &c in &ring {
            prop_assert_eq!(center.distance(c), radius);
        }
        prop_assert!(is_strictly_ascending(&ring));
    }

    // --- Disc ---
    // Checks that a disc of radius d has 3 * d * (d + 1) + 1 cells, all within
    // distance d, including the center, in strictly ascending order. With no
    // duplicates, these are all the cells within distance d.
    #[test]
    fn test_disc(center in any_cell(), radius in 0..50_u32) {
        let disc: Vec<Cell> = center.disc(radius).collect();
        let d = radius as usize;

        prop_assert_eq!(disc.len(), 3 * d * (d + 1) + 1);
        prop_assert!(disc.contains(&center));
        for &c in &disc {
            prop_assert!(center.distance(c) <= radius);
        }
        prop_assert!(is_strictly_ascending(&disc));
    }

    // --- Limits ---
    //check that s, neighbours and distance do not panic, and are still correct,
    //for every cell within MAX_MAP_RADIUS, which includes every cell of every Map
    #[test]
    fn test_no_overflow_inside_map(a in map_cell(), b in map_cell()) {
        prop_assert_eq!(a.q + a.r + a.s(), 0);
        for n in a.neighbours() {
            prop_assert_eq!(a.distance(n), 1);
        }
        prop_assert_eq!(a.distance(b), distance_max_formula(a, b));
    }

}

// --- Direction ---

//check that relative_angle is 3 exactly when the two offsets are opposite
#[test]
fn test_relative_angle_3_iff_opposite_offsets() {
    for a in ALL_DIRECTIONS {
        for b in ALL_DIRECTIONS {
            let (aq, ar) = a.offset();
            let opposite = b.offset() == (-aq, -ar);
            assert_eq!(a.relative_angle(b) == 3, opposite, "{a:?} vs {b:?}");
        }
    }
}

//check that relative_angle is 1 exactly when the two neighbours are adjacent
#[test]
fn test_relative_angle_1_iff_adjacent_neighbours() {
    let origin = Cell { q: 0, r: 0 };
    for a in ALL_DIRECTIONS {
        for b in ALL_DIRECTIONS {
            let adjacent = origin.neighbour(a).distance(origin.neighbour(b)) == 1;
            assert_eq!(a.relative_angle(b) == 1, adjacent, "{a:?} vs {b:?}");
        }
    }
}

// --- Limits ---

//check that opposite corners at the coordinate limit are at distance 2R:
//no Map has cells farther apart, and |dq| + |dr| + |ds| = 4R just fits in u32
#[test]
fn test_opposite_corners_at_coordinate_limit() {
    let radius = MAX_MAP_RADIUS;
    // Corner i is the cell radius steps along direction i.
    let corners = ALL_DIRECTIONS.map(|d| {
        let (dq, dr) = d.offset();
        Cell {
            q: dq * radius,
            r: dr * radius,
        }
    });
    for i in 0..6 {
        let (a, b) = (corners[i], corners[(i + 3) % 6]);
        assert_eq!(a.distance(b), 2 * radius.cast_unsigned(), "{a:?} vs {b:?}");
    }
}

// --- Documented panics ---

//check that s panics when q + r is i32::MIN, so -(q + r) does not fit in i32
#[test]
#[should_panic(expected = "s = -(q + r) overflows i32")]
fn test_s_panics_when_negation_overflows() {
    let _ = Cell { q: i32::MIN, r: 0 }.s();
}

//check that s panics when q + r itself does not fit in i32; with q = r = i32::MAX
//a wrapped sum would be -2, whose negation fits, so this does not pass by accident
#[test]
#[should_panic(expected = "s = -(q + r) overflows i32")]
fn test_s_panics_when_sum_overflows() {
    let _ = Cell {
        q: i32::MAX,
        r: i32::MAX,
    }
    .s();
}

//check that s does not panic when q + r is exactly -i32::MAX or i32::MAX
#[test]
fn test_s_at_the_limits() {
    assert_eq!(Cell { q: i32::MIN + 1, r: 0 }.s(), i32::MAX);
    assert_eq!(Cell { q: i32::MAX, r: 0 }.s(), -i32::MAX);
}

//check that neighbour panics when the step takes q out of the i32 range
#[test]
#[should_panic(expected = "neighbour q overflows i32")]
fn test_neighbour_panics_on_q_overflow() {
    let _ = Cell { q: i32::MAX, r: 0 }.neighbour(Direction::East);
}

//check that neighbour panics when the step takes r out of the i32 range
#[test]
#[should_panic(expected = "neighbour r overflows i32")]
fn test_neighbour_panics_on_r_overflow() {
    let _ = Cell { q: 0, r: i32::MAX }.neighbour(Direction::SouthEast);
}

//check that the same edge cell can still step back inside the i32 range
#[test]
fn test_neighbour_away_from_edge() {
    let c = Cell { q: i32::MAX, r: 0 };
    assert_eq!(c.neighbour(Direction::West), Cell { q: i32::MAX - 1, r: 0 });
}

//check that distance panics when |dq| + |dr| + |ds| does not fit in u32,
//even though both cells have a valid s
#[test]
#[should_panic(expected = "distance overflow u32")]
fn test_distance_panics_on_overflow() {
    let a = Cell {
        q: i32::MAX,
        r: -i32::MAX,
    };
    let b = Cell {
        q: -i32::MAX,
        r: i32::MAX,
    };
    let _ = a.distance(b);
}

//check that disc panics as soon as it is called, without yielding any cell
#[test]
#[should_panic(expected = "disc radius overflows i32")]
fn test_disc_panics_immediately_on_huge_radius() {
    let _ = Cell { q: 0, r: 0 }.disc(MAX_RADIUS + 1);
}

//check that disc accepts radius == i32::MAX (the limit is "> i32::MAX", not ">="),
//and yields its first cell
#[test]
fn test_disc_accepts_radius_i32_max() {
    let mut disc = Cell { q: 0, r: 0 }.disc(MAX_RADIUS);
    assert_eq!(disc.next(), Some(Cell { q: -i32::MAX, r: 0 }));
}

//check that disc panics when a yielded cell has a q that does not fit in i32
#[test]
#[should_panic(expected = "disc q overflows i32")]
fn test_disc_panics_on_q_overflow() {
    let c = Cell { q: i32::MAX, r: 0 };
    for _ in c.disc(1) {}
}

//check that disc panics when a yielded cell has an r that does not fit in i32
#[test]
#[should_panic(expected = "disc r overflows i32")]
fn test_disc_panics_on_r_overflow() {
    let c = Cell { q: 0, r: i32::MAX };
    for _ in c.disc(1) {}
}

//check that ring panics as soon as it is called, without yielding any cell
#[test]
#[should_panic(expected = "ring radius overflows i32")]
fn test_ring_panics_immediately_on_huge_radius() {
    let _ = Cell { q: 0, r: 0 }.ring(MAX_RADIUS + 1);
}

//check that ring accepts radius == i32::MAX (the limit is "> i32::MAX", not ">="),
//and yields its first cell
#[test]
fn test_ring_accepts_radius_i32_max() {
    let mut ring = Cell { q: 0, r: 0 }.ring(MAX_RADIUS);
    assert_eq!(ring.next(), Some(Cell { q: -i32::MAX, r: 0 }));
}

//check that ring panics when a yielded cell has a coordinate that does not fit
//in i32; the docs promise this only in debug builds
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn test_ring_panics_on_overflow_in_debug() {
    let c = Cell { q: i32::MAX, r: 0 };
    for _ in c.ring(1) {}
}
