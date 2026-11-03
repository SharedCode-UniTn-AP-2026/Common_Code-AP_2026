//! Uses integer arithmetic to find the nearest cells along
//! a line between two hexagonal cells.
use crate::cell::Cell;
use crate::map::Map;

/// Returns the squared cube-coordinate distance between a cell
/// center and a fractional sample point, scaled by `n²`.
///
/// Comparisons use integer arithmetic without floating-point rounding.
///
/// # Panics
///
/// May panic on arithmetic overflow for extreme coordinates.
fn squared_distance_to_sample(cell: Cell, q_num: i64, r_num: i64, n: i64) -> i64 {
    // Convert the cell coordinates to cube coordinates.
    let q = i64::from(cell.q);
    let r = i64::from(cell.r);
    let s = -q - r;

    // Calculate the third numerator of the fractional sample.
    let s_num = -q_num - r_num;

    // Calculate coordinate differences without dividing by n.
    let dq = q * n - q_num;
    let dr = r * n - r_num;
    let ds = s * n - s_num;

    // Return the squared distance scaled by n^2.
    // This preserves exact comparisons without floating-point errors.
    dq * dq + dr * dr + ds * ds
}

/// Returns the nearest candidate cells to a fractional sample point.
///
/// Cells with equal minimum squared distance are all included.
/// The result is calculated using integer comparisons.
///
/// # Panics
///
/// Panics if `n == 0` and may panic on arithmetic overflow.
fn nearest_candidates(q_num: i64, r_num: i64, n: i64) -> Vec<Cell> {
    assert!(n > 0);

    // Find the lower and upper integer values of q.
    let q0 = q_num.div_euclid(n);
    let q1 = q0 + i64::from(q_num.rem_euclid(n) != 0);

    // Find the lower and upper integer values of r.
    let r0 = r_num.div_euclid(n);
    let r1 = r0 + i64::from(r_num.rem_euclid(n) != 0);

    // Store the closest cells and the minimum distance found.
    let mut candidates = Vec::new();
    let mut min_distance = i64::MAX;

    // Check all combinations of nearby integer coordinates.
    for q in [q0, q1] {
        for r in [r0, r1] {
            // Convert coordinates back to the Cell type (i32).
            let (Ok(q), Ok(r)) = (i32::try_from(q), i32::try_from(r)) else {
                continue;
            };

            let cell = Cell { q, r };

            // Calculate the exact squared distance to the sample.
            let distance = squared_distance_to_sample(cell, q_num, r_num, n);

            if distance < min_distance {
                // A closer cell was found: replace previous candidates.
                min_distance = distance;
                candidates.clear();
                candidates.push(cell);
            } else if distance == min_distance && !candidates.contains(&cell) {
                // Keep equally close cells without adding duplicates.
                candidates.push(cell);
            }
        }
    }

    // Return all cells sharing the minimum distance.
    candidates
}

/// Returns the axial coordinate numerators of sample `k`
/// along the segment from `from` to `to`.
///
/// Both coordinates share denominator `n`.
///
/// # Panics
///
/// Panics if `n == 0` or `k` is outside `0..=n`.
/// May also panic on arithmetic overflow.
fn sample_numerators(from: Cell, to: Cell, k: i64, n: i64) -> (i64, i64) {
    assert!(n > 0);
    assert!(k >= 0 && k <= n);

    // Interpolate the q coordinate without dividing by n.
    let q_num = (n - k) * i64::from(from.q) + k * i64::from(to.q);

    // Interpolate the r coordinate without dividing by n.
    let r_num = (n - k) * i64::from(from.r) + k * i64::from(to.r);

    // Return the two numerators.
    (q_num, r_num)
}

/// Checks whether two cells have an unobstructed line of sight
/// within the given vision range.
///
/// Interior samples block visibility only when all their nearest
/// candidate cells are land. The endpoints' terrain is ignored.
///
/// This legacy implementation does not reject endpoints outside
/// the map.
///
/// # Panics
///
/// May panic on arithmetic overflow for extreme coordinates,
/// including overflow in `Cell::distance`.
#[must_use]
pub fn visible(map: &Map, from: Cell, to: Cell, vision_range: u32) -> bool {
    // Calculate the hexagonal distance between the two cells.
    let distance = from.distance(to);

    // Cells beyond the vision range are not visible.
    if distance > vision_range {
        return false;
    }

    let n = i64::from(distance);

    // Check only interior samples, excluding both endpoints.
    for k in 1..n {
        // Compute the exact fractional sample coordinates.
        let (q_num, r_num) = sample_numerators(from, to, k, n);

        // Find all cells equally closest to the sample.
        let candidates = nearest_candidates(q_num, r_num, n);

        // A sample blocks sight only when all candidates are land.
        if candidates.iter().all(|&cell| map.is_land(cell)) {
            return false;
        }
    }

    // No blocking samples were found.
    true
}

#[cfg(test)]
mod los_tests {
    use super::*;

    // Checks that two cells are exactly equidistant from a midpoint.
    #[test]
    fn test_squared_distance_midpoint() {
        let a = Cell { q: 1, r: -1 };
        let b = Cell { q: 1, r: 0 };

        // Midpoint (1, -0.5, -0.5)
        let q_num = 2;
        let r_num = -1;
        let n = 2;

        let distance_a = squared_distance_to_sample(a, q_num, r_num, n);

        let distance_b = squared_distance_to_sample(b, q_num, r_num, n);

        assert_eq!(distance_a, 2);
        assert_eq!(distance_b, 2);
        assert_eq!(distance_a, distance_b);
    }

    // Checks that an exact midpoint returns both nearest cells.
    #[test]
    fn test_nearest_candidates_midpoint() {
        // Point (1, -0.5, -0.5) represented exactly.
        let q_num = 2;
        let r_num = -1;
        let n = 2;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 2);

        assert!(candidates.contains(&Cell { q: 1, r: -1 }));
        assert!(candidates.contains(&Cell { q: 1, r: 0 }));
    }

    // Checks exact interpolation of sample points using integer numerators.
    #[test]
    fn test_sample_numerators() {
        let from = Cell { q: 0, r: 0 };
        let to = Cell { q: 2, r: 1 };

        let n = i64::from(from.distance(to));

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

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], Cell { q: 1, r: 0 });
    }

    // Checks that negative fractional coordinates return the correct nearest cell.
    #[test]
    fn test_nearest_candidates_negative() {
        // Point (-4/3, 1/3, 1) represented exactly.
        let q_num = -4;
        let r_num = 1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0], Cell { q: -1, r: 0 });
    }

    // Checks that a point on a shared vertex returns all three nearest cells.
    #[test]
    fn test_nearest_candidates_three_cells() {
        // Point (2/3, -1/3, -1/3) represented exactly.
        let q_num = 2;
        let r_num = -1;
        let n = 3;

        let candidates = nearest_candidates(q_num, r_num, n);

        assert_eq!(candidates.len(), 3);

        assert!(candidates.contains(&Cell { q: 0, r: 0 }));
        assert!(candidates.contains(&Cell { q: 1, r: -1 }));
        assert!(candidates.contains(&Cell { q: 1, r: 0 }));
    }

    // Checks that the optimized nearest-cell search matches an exhaustive search.
    #[test]
    fn test_nearest_candidates_brute_force() {
        // Test different denominators, including negative coordinates.
        for n in 1_i64..=6 {
            for q_num in (-2 * n)..=(2 * n) {
                for r_num in (-2 * n)..=(2 * n) {
                    let mut expected = Vec::new();
                    let mut min_distance = i64::MAX;

                    // Exhaustively check cells in a sufficiently large area.
                    for q in -4_i32..=4 {
                        for r in -4_i32..=4 {
                            let cell = Cell { q, r };

                            let distance = squared_distance_to_sample(cell, q_num, r_num, n);

                            if distance < min_distance {
                                min_distance = distance;
                                expected.clear();
                                expected.push(cell);
                            } else if distance == min_distance {
                                expected.push(cell);
                            }
                        }
                    }

                    // Compare with the optimized algorithm.
                    let mut actual = nearest_candidates(q_num, r_num, n);

                    // Sort because candidate order is not important.
                    expected.sort();
                    actual.sort();

                    assert_eq!(actual, expected, "Mismatch for q_num={q_num}, r_num={r_num}, n={n}");
                }
            }
        }
    }
}
