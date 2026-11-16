//! # Wind Generation
//!
//! This module provides deterministic wind direction generation based on a
//! seed and a round number.
//!
//! Wind directions are represented as a cyclic space containing six possible
//! directions. The generated sequence is divided into fixed-size blocks,
//! each having a deterministic starting and ending anchor.
//!
//! ## Algorithm
//!
//! Each block is generated using a combination of probabilistic selection
//! and direction visitation tracking.
//!
//! - **Anchors:** Each block has deterministic anchor directions derived
//!   from the seed and block index.
//! - **Direction tracking:** The algorithm tracks visited directions and
//!   their respective ages to encourage exploration of all directions.
//! - **Probabilistic selection:** Older directions have a higher probability
//!   of being selected, while allowing the wind to remain in its current
//!   direction.
//! - **Block continuity:** Each block is constrained to reach the next
//!   block's anchor, ensuring continuity between consecutive blocks.
//!
//! ## Determinism
//!
//! Given the same seed and round, the generated wind direction is intended
//! to be reproducible.
//!
//! ## Examples
//!
//! ```
//! # use ap_2026_common_code_wind::wind_at;
//!
//! let seed = 12345;
//! let direction = wind_at(seed, 30);
//! ```

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

use geometry::Direction;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use shared::{Round, Seed};
use std::cmp::min;
use std::hash::{DefaultHasher, Hash, Hasher};

// The time frame in which the wind needs to cover all six directions
const WINDOW_SIZE: u32 = 60;
// The size of the block used for computing the wind direction
// Integer division should always floor
const BLOCK_SIZE: u32 = (WINDOW_SIZE - 1) / 2;
// Number of available directions
const DIRECTION_NUMBER: u32 = 6;

#[must_use]
/// Calculates the wind direction for a given round.
///
/// # Arguments
///
/// * `seed` - The seed used to generate the wind sequence.
/// * `round` - The round for which the wind direction is calculated.
///
/// # Returns
///
/// The [`Direction`] associated with the specified round.
///
/// # Examples
///
/// ```
/// # use ap_2026_common_code_wind::wind_at;
/// # use geometry::Direction;
/// # let dummy_seed = 42;
/// let direction: Direction = wind_at(dummy_seed, 30);
/// ```
pub fn wind_at(seed: Seed, round: Round) -> Direction {
    let block_number = (round / BLOCK_SIZE) as usize;

    u32_to_direction(block_at(seed, block_number)[(round % BLOCK_SIZE) as usize])
}

/// Calculates `BLOCK_SIZE` directions for a given seed and block number, guaranteeing that each
/// `Direction` variant appears at least once inside the block, which itself guarantees the same
/// result for each `WINDOWS_SIZE` long sequence of wind directions.
/// The generation of wind directions is probabilistic.
/// If the selected direction would make reaching the ending anchor impossible within the remaining
/// rounds, an alternative direction is enforced.
///
/// # Arguments
///
/// * `seed` - The seed used to initialize the random number generator and calculate
///   the block's anchor directions.
/// * `block_number` - The index of the block to generate.
///
/// # Returns
///
/// An array containing `BLOCK_SIZE` wind directions.
///
/// # Panics
///
/// Panics if a generated direction index cannot be converted to `u32`, which should
/// not occur when the direction index is within the valid range.
fn block_at(seed: Seed, block_number: usize) -> [u32; BLOCK_SIZE as usize] {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut block = [0; BLOCK_SIZE as usize];
    let end_direction = calculate_anchor(seed, block_number + 1);

    let mut visited = [false; DIRECTION_NUMBER as usize];
    let mut age = [0; DIRECTION_NUMBER as usize];

    let mut current = calculate_anchor(seed, block_number);

    visited[current as usize] = true;

    for i in 0..BLOCK_SIZE {
        let remaining_rounds = BLOCK_SIZE - i;

        let mut choice = probabilistic_choice(age, current, end_direction, &mut rng, remaining_rounds);

        // The -1 is due to the fact that we need to consider the current rounds movement
        if visit_time(safe_addition(current.cast_signed(), choice), visited, end_direction) >= remaining_rounds - 1 {
            choice = enforced_direction(current, visited, end_direction);
        }

        let idx = safe_addition(current.cast_signed(), choice) as usize;
        visited[idx] = true;
        age[idx] = 0;

        block[i as usize] = current;
        current = u32::try_from(idx).expect("Index should be a value between 0 and 6");

        for j in 0..DIRECTION_NUMBER {
            if j != current {
                age[j as usize] += 1;
            }
        }
    }

    block
}

/// Performs addition with wraparound over the six possible directions.
///
/// The result is normalized to the range `0..DIRECTION_NUMBER`, ensuring that negative
/// and positive additions both produce a valid direction index.
///
/// # Arguments
///
/// * `x` - The left-hand operand.
/// * `y` - The right-hand operand.
///
/// # Returns
///
/// The result of `(x + y) mod DIRECTION_NUMBER`, represented as a `u32` in the range `0..6`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(safe_addition(4, 1), 5);
/// assert_eq!(safe_addition(5, 1), 0);
/// assert_eq!(safe_addition(0, -1), 5);
/// assert_eq!(safe_addition(2, -4), 4);
/// ```
fn safe_addition(x: i32, y: i32) -> u32 {
    (x + y).rem_euclid(6).cast_unsigned()
}

/// Converts an u32 between `0..6` to a [`Direction`].
///
/// # Arguments
///
/// * `x` - The u32 to convert to a [`Direction`].
///
/// # Returns
///
/// The direction associated with the value of `x`
///
/// # Panics
/// If `x` is not in the range `0..6`
///
/// # Examples
/// ```ignore
/// use geometry::Direction;
/// let d = u32_to_direction(0);
/// assert_eq!(d, Direction::East)
/// ```
fn u32_to_direction(x: u32) -> Direction {
    match x {
        0 => Direction::East,
        1 => Direction::NorthEast,
        2 => Direction::NorthWest,
        3 => Direction::West,
        4 => Direction::SouthWest,
        5 => Direction::SouthEast,
        _ => panic!("Invalid direction: {x}"),
    }
}

/// Calculates the relative angle between two [`Direction`] represented as `i32`
///
/// # Arguments
///
/// * `x` - The first [`Direction`]
/// * `y` - The second [`Direction`]
///
/// # Returns
///
/// An `i32` representing the relative angle between `x` and `y`
///
/// # Examples
///
/// ```ignore
/// assert_eq!(relative_angle_i32(0, 2), 2);
/// assert_eq!(relative_angle_i32(2, 5), 3);
/// assert_eq!(relative_angle_i32(3, 1), 2);
/// ```
fn relative_angle_i32(x: i32, y: i32) -> i32 {
    let d1 = u32_to_direction(x.cast_unsigned());
    let d2 = u32_to_direction(y.cast_unsigned());

    i32::from(d1.relative_angle(d2).cast_signed())
}

/// Calculates a deterministic anchor direction for a given block number.
///
/// The anchor is derived by hashing the seed and block number, then mapping
/// the resulting hash to one of the available directions.
///
/// # Arguments
///
/// * `seed` - The seed used to generate the anchor.
/// * `block_number` - The index of the block.
///
/// # Returns
///
/// A direction index in the range `0..DIRECTION_NUMBER`.
///
/// # Examples
///
/// ```ignore
/// let anchor = calculate_anchor(12345, 0);
/// assert!(anchor < DIRECTION_NUMBER);
/// ```
fn calculate_anchor(seed: Seed, block_number: usize) -> u32 {
    let to_hash = format!("{seed}:{block_number}");
    let mut hasher = DefaultHasher::new();
    to_hash.hash(&mut hasher);
    let hash = hasher.finish();

    // Cast as u32 to keep the same type as DIRECTION_NUMBER
    u32::try_from(hash % u64::from(DIRECTION_NUMBER))
        .expect("Direction number is a u32, so the modulo should return a u32")
}

/// Determines the numerical value of the preferred `Direction` to move towards the ending anchor.
///
/// If the current direction is already equal to the ending anchor or one of
/// its adjacent directions, the current direction number is returned. Otherwise,
/// the closest adjacent direction number to the ending anchor is selected.
///
/// # Arguments
///
/// * `current` - The current direction number.
/// * `end` - The ending anchor direction number.
///
/// # Returns
///
/// The preferred target direction number, represented as an `i32`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(calculate_end_target(0, 3), 2);
/// assert_eq!(calculate_end_target(2, 2), 2);
/// ```
fn calculate_end_target(current: i32, end: i32) -> i32 {
    let end_plus = safe_addition(end, 1).cast_signed();
    let end_minus = safe_addition(end, -1).cast_signed();

    if current == end || current == end_plus || current == end_minus {
        return current;
    }

    if relative_angle_i32(current, end_plus) < relative_angle_i32(current, end_minus) {
        end_plus
    } else {
        end_minus
    }
}

/// Calculates the minimum number of movements required to visit all unvisited
/// directions and reach a valid `Direction` with respect to the relative ending anchor.
///
/// The calculation considers the two possible traversal orders of the
/// unvisited directions around the cyclic direction space and chooses the
/// shorter route.
///
/// # Arguments
///
/// * `current` - The current direction index.
/// * `visited` - An array indicating which directions have already been visited.
/// * `end` - The ending anchor direction.
///
/// # Returns
///
/// The minimum number of movements required to visit all remaining directions
/// and reach a valid `Direction` with respect to the relative ending anchor.
///
/// # Examples
///
/// ```ignore
/// let visited = [false, false, false, true, true, false];
/// assert_eq!(visit_time(3, visited, 3), 5);
/// ```
fn visit_time(current: u32, visited: [bool; DIRECTION_NUMBER as usize], end: u32) -> u32 {
    let current = current.cast_signed();
    let end = end.cast_signed();

    if let Some((start, last)) = unvisited_endpoints(visited) {
        min(
            relative_angle_i32(current, start)
                + relative_angle_i32(start, last)
                + relative_angle_i32(last, calculate_end_target(last, end)),
            relative_angle_i32(current, last)
                + relative_angle_i32(last, start)
                + relative_angle_i32(start, calculate_end_target(start, end)),
        )
        .cast_unsigned()
    } else {
        relative_angle_i32(current, calculate_end_target(current, end)).cast_unsigned()
    }
}

/// Finds the first and last unvisited directions in the cyclic direction space.
///
/// This function scans through the direction range, identifying the boundaries (start and last)
/// of the unvisited direction segments.
///
/// # Arguments
///
/// * `visited` - An array indicating which directions have already been visited.
///
/// # Returns
///
/// A tuple `Option<(i32, i32)>` containing:
/// * `Some((start, last))` - The respective indexes of the first and last unvisited direction encountered.
/// * `None` - If all directions have already been visited.
///
/// # Panics
///
/// Panics if a direction index cannot be successfully converted to an `i32`,
/// which should not occur as indices are kept within the valid range of `0..DIRECTION_NUMBER`.
///
/// # Examples
///
/// ```ignore
/// let visited = [false, false, false, true, true, false];
/// let Some(start, last) = unvisited_endpoints(visited, 3);
/// assert!(start.is_some());
/// assert!(last.is_some());
/// ```
fn unvisited_endpoints(visited: [bool; DIRECTION_NUMBER as usize]) -> Option<(i32, i32)> {
    let mut start: Option<i32> = None;
    let mut last: Option<i32> = None;
    let mut i: usize = 0;

    // Find the first unvisited node and count unvisited nodes
    for _ in 0..DIRECTION_NUMBER {
        i = (i + 1).rem_euclid(6);
        if !visited[i] {
            if start.is_none() {
                start = Some(i32::try_from(i).expect("i is between 0 and 6"));
            }

            last = Some(i32::try_from(i).expect("i is between 0 and 6"));
        }
    }

    if let Some(start) = start
        && let Some(last) = last
    {
        Some((start, last))
    } else {
        None
    }
}

/// Determines the mandatory movement direction towards the closest unvisited acceptable endpoint.
///
/// Evaluates the two possible traversal paths across the unvisited directions
/// (via `start` or via `last`) and selects the direction that minimizes the total
/// movement cost required to cover all remaining unvisited nodes and reach an acceptable endpoint.
///
/// # Arguments
///
/// * `current` - The current direction index.
/// * `visited` - An array indicating which directions have already been visited.
/// * `end` - The ending anchor direction.
///
/// # Returns
///
/// The forced movement offset (`1`, `-1`, or `0`) towards the optimal unvisited endpoint,
/// represented as an `i32`. Returns `0` (or retains the current direction) if all
/// directions have already been visited.
///
/// # Examples
///
/// ```ignore
/// let visited = [false, false, false, true, true, false];
/// let forced = enforced_direction(3, visited, 3);
/// assert!(forced >= -1 && forced <= 1);
/// ```
fn enforced_direction(current: u32, visited: [bool; DIRECTION_NUMBER as usize], end: u32) -> i32 {
    let current = current.cast_signed();
    let end = end.cast_signed();

    if let Some((start, last)) = unvisited_endpoints(visited) {
        if relative_angle_i32(current, start)
            + relative_angle_i32(start, last)
            + relative_angle_i32(last, calculate_end_target(last, end))
            < relative_angle_i32(current, last)
                + relative_angle_i32(last, start)
                + relative_angle_i32(start, calculate_end_target(start, end))
        {
            towards(current, start)
        } else {
            towards(current, last)
        }
    } else {
        towards(current, calculate_end_target(current, end))
    }
}

/// Selects a movement probabilistically based on the age of each direction.
///
/// Older directions receive a higher probability of being selected, encouraging
/// directions that have not been visited recently to be revisited. The current
/// direction receives an additional probability of remaining unchanged.
///
/// The selected movement is adjusted when necessary to ensure that the ending
/// anchor remains reachable within the remaining rounds.
///
/// # Arguments
///
/// * `age` - An array containing the number of rounds each direction has remained unvisited.
/// * `current` - The current direction number.
/// * `end` - The ending anchor direction number.
/// * `rng` - The random number generator used to break ties.
/// * `remaining` - The number of rounds remaining in the current block.
///
/// # Returns
///
/// The movement offset selected by the probabilistic algorithm.
///
/// # Examples
///
/// ```ignore
/// let age = [0, 1, 2, 3, 4, 5];
/// let mut rng = StdRng::seed_from_u64(42);
///
/// let choice = probabilistic_choice(age, 0, 3, &mut rng, 10);
/// assert!(choice >= -1 && choice <= 1);
/// ```
fn probabilistic_choice(
    age: [i32; DIRECTION_NUMBER as usize],
    current: u32,
    end: u32,
    rng: &mut StdRng,
    remaining: u32,
) -> i32 {
    let mut probabilities = [0f64; DIRECTION_NUMBER as usize];
    // let hold_probability = rng.next_u64() % 31;
    let hold_probability = rng.random_range(0..=30);

    let total_age = f64::from(age.iter().sum::<i32>());
    for (i, a) in age.iter().enumerate() {
        probabilities[i] = f64::from(*a) / total_age;
    }

    probabilities[current as usize] = f64::from(hold_probability) / 100.0;

    // let chosen = rng.next_u64() % (100 + hold_probability + 1);
    let chosen: u32 = rng.random_range(0..=(100 + hold_probability));

    let mut i = 0;
    let mut sum = probabilities[0] * 100.0;

    let mut tow;
    while sum < f64::from(chosen) && i < 5 {
        i += 1;
        sum += probabilities[i] * 100.0;
    }

    let current = current.cast_signed();
    let end = end.cast_signed();
    let remaining = remaining.cast_signed();

    tow = towards(current, i32::try_from(i).expect("Could not convert from i32"));

    let end_target = calculate_end_target(current, end);

    // let tow_current = ((current as i32) + tow).rem_euclid(6);
    if relative_angle_i32(current, end_target) >= remaining - 2 {
        tow = towards(current, end_target);
    }

    tow
}

/// Determines the shortest movement towards a target direction.
///
/// Since directions form a cyclic space, movement can wrap around either end
/// of the direction range. If both directions are equally distant, movement
/// towards the negative direction is preferred.
///
/// # Arguments
///
/// * `current` - The current direction index.
/// * `target` - The target direction index.
///
/// # Returns
///
/// * `1` - Move one direction forward.
/// * `-1` - Move one direction backward.
/// * `0` - The current direction is already the target.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(towards(0, 1), 1);
/// assert_eq!(towards(0, 5), -1);
/// assert_eq!(towards(2, 2), 0);
/// ```
fn towards(current: i32, target: i32) -> i32 {
    if current == target {
        return 0;
    }

    if relative_angle_i32(safe_addition(current, 1).cast_signed(), target)
        < relative_angle_i32(safe_addition(current, -1).cast_signed(), target)
    {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction::NorthEast;
    use crate::{
        BLOCK_SIZE, DIRECTION_NUMBER, block_at, calculate_anchor, safe_addition, towards, visit_time, wind_at,
    };
    use rand::{Rng, rng};

    // Tests the towards function correctness
    #[test]
    fn towards_test() {
        let dir1 = 1;
        let target = 5;

        assert_eq!(towards(dir1, target), -1);

        let dir2 = 0;
        assert_eq!(towards(dir2, target), -1);

        let dir3 = 0;
        assert_eq!(towards(target, dir3), 1);
    }

    // Tests that a block starts from the start anchor and ends within one direction of the end anchor
    #[test]
    fn block_integrity_test() {
        let seed = rng().next_u64();
        println!("{seed}");

        let block = block_at(seed, 0);
        let block2 = block_at(seed, 1);

        let second_anchor = calculate_anchor(seed, 1);
        println!("Second anchor: {second_anchor}");
        println!("{block:?}");

        assert_eq!(block[0], calculate_anchor(seed, 0));
        assert!(
            block[BLOCK_SIZE as usize - 1] == safe_addition(second_anchor.cast_signed(), 1)
                || block[BLOCK_SIZE as usize - 1] == safe_addition(second_anchor.cast_signed(), -1)
                || block[BLOCK_SIZE as usize - 1] == second_anchor
        );
        assert_eq!(block2[0], second_anchor);
    }

    // Tests that the wind_at function remains deterministic
    #[test]
    fn wind_at_test() {
        let seed = 1_528_890;

        // The start direction is 5
        assert_eq!(calculate_anchor(seed, 0), 5);

        // The end direction is 4
        assert_eq!(calculate_anchor(seed, 1), 4);

        let block = block_at(seed, 0);
        assert_eq!(
            [
                5, 0, 1, 0, 1, 2, 1, 0, 5, 4, 3, 2, 2, 1, 0, 5, 0, 1, 1, 0, 1, 2, 1, 2, 1, 1, 2, 3, 3
            ],
            block
        );

        let wind = wind_at(seed, 4);
        assert_eq!(wind, NorthEast);
    }

    // Tests the visit time function correctness
    #[test]
    fn visit_time_test() {
        assert_eq!(visit_time(3, [false, false, false, true, true, false], 3), 5);
        assert_eq!(visit_time(1, [true, true, true, true, true, false], 5), 2);

        // Possible answers are 2 and 4, the shortest route is 2 so we take it.
        assert_eq!(visit_time(1, [false, true, true, true, true, false], 5), 2);
        assert_eq!(visit_time(4, [true, false, false, false, true, true], 0), 3);
    }

    // Tests that a block visits all possible directions
    #[test]
    fn all_directions_visited_test() {
        //for x in 0..3_000_000 {
        let seed = rng().next_u64();

        // let seed = 14208738534138294339;
        println!("{seed}");

        let block = block_at(seed, 0);
        let mut visited = [false; DIRECTION_NUMBER as usize];

        println!("{block:?}");
        // Block is only 30 items
        for x in &block {
            visited[*x as usize] = true;
        }
        assert_eq!(visited.iter().filter(|x| **x).count(), 6);
        //}
    }
}
