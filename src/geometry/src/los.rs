//! Uses integer arithmetic to find the nearest cells along
//! a line between two hexagonal cells.
use crate::cell::Cell;
use crate::map::Map;

/// Returns the squared cube-coordinate distance scaled by `n^2`
fn squared_distance(dq: i32, dr: i32) -> i64 {
    let dq = i64::from(dq);
    let dr = i64::from(dr);
    let ds = -dq - dr;

    dq * dq + dr * dr + ds * ds
}

/// Returns up to 2 nearest candidate cells to a fractional sample point.
///
/// The function takes the r and q interpolated from the [`sample_numerators`] and performs Euclidean
/// division on them to find the closest cells (up to 2 cells for every pair of coordinates).
/// Using Euclidean division, we eliminate the risk of floating point inaccuracies.
fn nearest_candidates(q_num: i64, r_num: i64, n: i32) -> [Option<Cell>; 2] {
    debug_assert!(n > 0);
    let n_i64 = i64::from(n);

    // Euclidean division gives the lower integer coordinate (floor)
    // and the remainder in `0..n`.
    let q0 = q_num.div_euclid(n_i64) as i32;
    let rem_q = q_num.rem_euclid(n_i64) as i32;

    let r0 = r_num.div_euclid(n_i64) as i32;
    let rem_r = r_num.rem_euclid(n_i64) as i32;

    // For `q0`, `q0 * n - q_num` is simply `-rem_q`.
    // For `q0 + 1` (if rem_q != 0), `(q0 + 1) * n - q_num` is `n - rem_q`.
    let q_steps = if rem_q == 0 {
        // The division is exact, no remainder
        [(q0, 0), (q0, 0)]
    } else {
        // We need to consider two cells.
        // How close the two cells are to the point is determined by the remainder
        [(q0, -rem_q), (q0 + 1, n - rem_q)]
    };

    let r_steps = if rem_r == 0 {
        [(r0, 0), (r0, 0)]
    } else {
        [(r0, -rem_r), (r0 + 1, n - rem_r)]
    };

    let mut candidates = [None; 2];
    let mut last_index = 0;
    let mut min_distance = i64::MAX;

    // Taking the nearest candidates
    for (q, dq) in q_steps {
        for (r, dr) in r_steps {
            let cell = Cell { q, r };
            let distance = squared_distance(dq, dr);

            if distance < min_distance {
                min_distance = distance;
                candidates = [Some(cell), None];
                last_index = 1;
            } else if distance == min_distance && !candidates.contains(&Some(cell)) {
                // if we have two cell at the same distance (border between the cell) we take both
                candidates[last_index] = Some(cell);
                last_index += 1;
            }
        }
    }

    candidates
}

/// Returns the axial coordinate (`q_num`,`r_num`) numerators of sample `k`
/// along the segment from `from` to `to`.
///
/// Both coordinates share denominator `n`.
fn sample_numerators(from: Cell, to: Cell, k: i32, n: i32) -> (i64, i64) {
    debug_assert!(n > 0);
    debug_assert!(k >= 0 && k <= n);

    let n_minus_k = i64::from(n - k);
    let k = i64::from(k);

    let q_num = n_minus_k * i64::from(from.q) + k * i64::from(to.q);
    let r_num = n_minus_k * i64::from(from.r) + k * i64::from(to.r);

    (q_num, r_num)
}

/// Checks whether two cells have an unobstructed line of sight
/// within the given vision range.
///
/// Interior samples block visibility only when all their nearest
/// candidate cells are land. The endpoints' terrain is ignored.
#[must_use]
pub fn visible(map: &Map, from: Cell, to: Cell, vision_range: u32) -> bool {
    // Check if the cells are inside the map
    if !map.contains(from) || !map.contains(to) {
        return false;
    }

    let distance = from.distance(to);

    // Cells beyond the vision range are not visible.
    if distance > vision_range {
        return false;
    }

    // Distance can safely be rappresented as an i32 (bounded by 2*radius)
    let n = distance as i32;

    for k in 1..n {
        let (q_num, r_num) = sample_numerators(from, to, k, n);

        let candidates = nearest_candidates(q_num, r_num, n);

        // A sample blocks sight only when all candidates are land.
        if candidates.iter().flatten().all(|&cell| map.is_land(cell)) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    // Checks that an exact midpoint returns both nearest cells.
    #[test]
    fn test_nearest_candidates_midpoint() {
        // Point (1, -0.5, -0.5) represented exactly.
        let q_num = 2;
        let r_num = -1;
        let n = 2;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 2);

        assert!(candidates.contains(&Some(Cell { q: 1, r: -1 })));
        assert!(candidates.contains(&Some(Cell { q: 1, r: 0 })));
    }

    // Checks exact interpolation of sample points using integer numerators.
    #[test]
    fn test_sample_numerators() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 1 };

        let n = from.distance(to) as i32;

        assert_eq!(n, 3);

        // First intermediate sample: (2/3, 1/3)
        assert_eq!(sample_numerators(from, to, 1, n), (2, 1));

        // Second intermediate sample: (4/3, 2/3)
        assert_eq!(sample_numerators(from, to, 2, n), (4, 2));

        // The endpoints must match the original cells.
        assert_eq!(sample_numerators(from, to, 0, n), (0, 0));
        assert_eq!(sample_numerators(from, to, 3, n), (6, 3));
    }

    // Checks that a fractional sample has exactly one nearest cell.
    #[test]
    fn test_nearest_candidates_single() {
        let q_num = 2;
        let r_num = 1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.iter().flatten().collect::<Vec<&Cell>>().len(), 1);
        assert_eq!(candidates[0], Some(Cell { q: 1, r: 0 }));
    }

    // Checks that negative fractional coordinates return the correct nearest cell.
    #[test]
    fn test_nearest_candidates_negative() {
        // Point (-4/3, 1/3, 1) represented exactly.
        let q_num = -4;
        let r_num = 1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.iter().flatten().collect::<Vec<&Cell>>().len(), 1);
        assert_eq!(candidates[0], Some(Cell { q: -1, r: 0 }));
    }
}
