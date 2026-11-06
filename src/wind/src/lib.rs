//! Module containing the `wind_at` function

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

#[deprecated(note = "Change to official geometry implementation once merged")]
type Seed = u64;

#[deprecated(note = "Change to official geometry implementation once merged")]
type Round = u32;

#[allow(missing_docs)]
#[deprecated(note = "Change to official geometry implementation once merged")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
/// Temporary implementation of the `Direction` enum to test the code
pub enum Direction {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            0 => East,
            1 => NorthEast,
            2 => NorthWest,
            3 => West,
            4 => SouthWest,
            5 => SouthEast,
            _ => panic!("Invalid direction: {value}"),
        }
    }
}

#[deprecated(note = "Change to official geometry implementation once merged")]
/// Calculates the relative angle between two `Direction`s represented in integer form
fn relative_angle(current: i32, target: i32) -> i32 {
    (current - target).abs().min(6 - (current - target).abs()).rem_euclid(6)
}

use crate::Direction::{East, NorthEast, NorthWest, SouthEast, SouthWest, West};
use rand::prelude::IndexedRandom;
use rand::rngs::StdRng;
use rand::{Rng, RngExt, SeedableRng};
use std::cmp::min;
use std::hash::{DefaultHasher, Hash, Hasher};

/// The time frame in which the wind needs to cover all six directions
const WINDOW_SIZE: u32 = 60;

// Integer division should always floor
const BLOCK_SIZE: u32 = (WINDOW_SIZE - 1) / 2;
const DIRECTION_NUMBER: u32 = 6;

#[must_use]
pub fn wind_at(seed: Seed, round: Round) -> Direction {
    let block_number = (round / BLOCK_SIZE) as usize;

    Direction::from(block_at(seed, block_number)[(round % BLOCK_SIZE) as usize])
}

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
            choice = towards_oldest_choice(age, current, end_direction, &mut rng, remaining_rounds);
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

fn safe_addition(x: i32, y: i32) -> u32 {
    (x + y).rem_euclid(6).cast_unsigned()
}

fn calculate_anchor(seed: Seed, block_number: usize) -> u32 {
    let to_hash = format!("{seed}:{block_number}");
    let mut hasher = DefaultHasher::new();
    to_hash.hash(&mut hasher);
    let hash = hasher.finish();

    // Cast as u32 to keep the same type as DIRECTION_NUMBER
    u32::try_from(hash % u64::from(DIRECTION_NUMBER))
        .expect("Direction number is a u32, so the modulo should return a u32")
}

fn calculate_end_target(current: i32, end: i32) -> i32 {
    let end_plus = safe_addition(end, 1).cast_signed();
    let end_minus = safe_addition(end, -1).cast_signed();

    if current == end || current == end_plus || current == end_minus {
        return current;
    }

    if relative_angle(current, end_plus) < relative_angle(current, end_minus) {
        end_plus
    } else {
        end_minus
    }
}

fn visit_time(current: u32, visited: [bool; DIRECTION_NUMBER as usize], end: u32) -> u32 {
    let mut start: Option<i32> = None;
    let mut last: Option<i32> = None;
    let mut unvisited = 0;
    let mut j = current as usize;

    // Find the first unvisited node and count unvisited nodes
    for i in 0..DIRECTION_NUMBER {
        j = (j + 1).rem_euclid(6);
        if !visited[j] {
            if start.is_none() {
                start = Some(i32::try_from(j).expect("j is between 0 and 6"));
            }

            last = Some(i32::try_from(j).expect("j is between 0 and 6"));

            unvisited += 1;
        }
    }

    let current = current.cast_signed();
    let end = end.cast_signed();

    // If start is different from None, then so is last by construction
    match (start, last) {
        (Some(start), Some(last)) => min(
            relative_angle(current, start) + (unvisited - 1) + relative_angle(last, calculate_end_target(last, end)),
            relative_angle(current, last) + (unvisited - 1) + relative_angle(start, calculate_end_target(start, end)),
        )
        .cast_unsigned(),
        _ => 0,
    }
}

fn towards_oldest_choice(
    age: [i32; DIRECTION_NUMBER as usize],
    current: u32,
    end: u32,
    rng: &mut StdRng,
    remaining: u32,
) -> i32 {
    let max_value = *age.iter().max().unwrap_or(&0);
    let max_idx: Vec<usize> = age
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == max_value)
        .map(|(x, _)| x)
        .collect();

    let oldest_dir: i32 = i32::try_from(
        max_idx
            .choose(rng)
            .expect("Could not choose one of the oldest")
            .cast_signed(),
    )
    .expect("Could not convert oldest dir to i32");

    let current = current.cast_signed();
    let end = end.cast_signed();
    let remaining = remaining.cast_signed();

    let mut tow = towards(current, oldest_dir);

    let end_target = calculate_end_target(safe_addition(current, tow).cast_signed(), end);

    // let tow_current = ((current as i32) + tow).rem_euclid(6);
    if relative_angle(safe_addition(current, tow).cast_signed(), end_target) >= remaining {
        tow = towards(current, end_target);
    }

    tow
}

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
    if relative_angle(current, end_target) >= remaining - 2 {
        tow = towards(current, end_target);
    }

    tow
}

fn towards(current: i32, target: i32) -> i32 {
    if current == target {
        return 0;
    }

    if relative_angle(safe_addition(current, 1).cast_signed(), target)
        < relative_angle(safe_addition(current, -1).cast_signed(), target)
    {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction::{NorthEast, West};
    use crate::{
        BLOCK_SIZE, DIRECTION_NUMBER, block_at, calculate_anchor, safe_addition, towards, visit_time, wind_at,
    };
    use rand::{Rng, rng};

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

    #[test]
    fn visit_time_test() {
        assert_eq!(visit_time(3, [false, false, false, true, true, false], 3), 5);
        assert_eq!(visit_time(1, [true, true, true, true, true, false], 5), 2);

        // Possible answers are 2 and 4, the shortest route is 2 so we take it.
        assert_eq!(visit_time(1, [false, true, true, true, true, false], 5), 2);
        assert_eq!(visit_time(4, [true, false, false, false, true, true], 0), 3);
    }

    #[test]
    fn all_directions_visited_test() {
        // for x in 0..100000 {
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
        // }
    }
}
