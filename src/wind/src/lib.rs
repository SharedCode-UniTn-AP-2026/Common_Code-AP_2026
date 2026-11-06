//! //! Module containing the wind_at function

//! Do not change these
#![deny(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(missing_docs)]

#[deprecated(note = "Change to official geometry implementation once merged")]
type Seed = u64;

#[deprecated(note = "Change to official geometry implementation once merged")]
type Round = u32;

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
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::min;
use std::hash::{DefaultHasher, Hash, Hasher};

/// The window size
const WINDOW_SIZE: u32 = 60;
const BLOCK_SIZE: usize = ((WINDOW_SIZE - 1) as f32 / 2f32).floor() as usize;
const DIRECTION_NUMBER: u32 = 6;

pub fn wind_at(seed: Seed, round: Round) -> Direction {
    let block_number = (round as f32 / BLOCK_SIZE as f32).floor() as usize;

    Direction::from(block_at(seed, block_number)[(round % (BLOCK_SIZE as u32)) as usize])
}

fn block_at(seed: Seed, block_number: usize) -> [u32; BLOCK_SIZE] {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut block = [0; BLOCK_SIZE];
    let end_direction = calculate_anchor(seed, block_number + 1);

    let mut visited = [false; DIRECTION_NUMBER as usize];
    let mut age = [0; DIRECTION_NUMBER as usize];

    let mut current = calculate_anchor(seed, block_number);

    visited[current as usize] = true;

    for i in 0..BLOCK_SIZE {
        let remaining_rounds = (BLOCK_SIZE - i) as u32;

        let mut choice = probabilistic_choice(age, current, end_direction, &mut rng, remaining_rounds);

        // The -1 is due to the fact that we need to consider the current rounds movement
        if visit_time((current as i32 + choice).rem_euclid(6) as u32, visited, end_direction) >= remaining_rounds - 1 {
            choice = towards_oldest_choice(age, current, end_direction, &mut rng, remaining_rounds);
        }

        let idx = (current as i32 + choice).rem_euclid(6) as usize;
        visited[idx] = true;
        age[idx] = 0;

        block[i] = current;
        current = idx as u32;

        for j in 0..DIRECTION_NUMBER {
            if j != current {
                age[j as usize] += 1
            }
        }
    }

    block
}

fn calculate_anchor(seed: Seed, block_number: usize) -> u32 {
    let to_hash = format!("{}:{}", seed, block_number);
    let mut hasher = DefaultHasher::new();
    to_hash.hash(&mut hasher);
    let hash = hasher.finish();

    // Cast as u32 to keep the same type as DIRECTION_NUMBER
    let current = (hash % DIRECTION_NUMBER as u64) as u32;
    current
}

fn calculate_end_target(current: i32, end: i32) -> i32 {
    if current == end || current == (end + 1).rem_euclid(6) || current == (end - 1).rem_euclid(6) {
        return current;
    }

    if relative_angle(current, (end + 1).rem_euclid(6)) < relative_angle(current, (end - 1).rem_euclid(6)) {
        (end + 1).rem_euclid(6)
    } else {
        (end - 1).rem_euclid(6)
    }
}

fn visit_time(current: u32, visited: [bool; DIRECTION_NUMBER as usize], end: u32) -> u32 {
    let mut start = None;
    let mut unvisited = 0;
    let mut last = None;
    let mut j = current as usize;

    // Find the first unvisited node and count unvisited nodes
    for i in 0..DIRECTION_NUMBER {
        j = (j + 1).rem_euclid(6);
        if !visited[j] {
            if start.is_none() {
                start = Some(j);
            }

            last = Some(j);

            unvisited += 1;
        }
    }

    // If start is different from None, then so is last by construction
    match (start, last) {
        (Some(start), Some(last)) => min(
            relative_angle(current as i32, start as i32)
                + (unvisited - 1)
                + relative_angle(last as i32, calculate_end_target(last as i32, end as i32)),
            relative_angle(current as i32, last as i32)
                + (unvisited - 1)
                + relative_angle(start as i32, calculate_end_target(start as i32, end as i32)),
        ) as u32,
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
    let max_value = age.iter().max().unwrap_or(&0).clone();
    let max_idx: Vec<usize> = age
        .iter()
        .enumerate()
        .filter(|(_, x)| **x == max_value)
        .map(|(x, _)| x)
        .collect();

    let oldest_dir = max_idx[rng.next_u64() as usize % max_idx.len()];

    let mut tow = towards(current as i32, oldest_dir as i32);

    let end_target = calculate_end_target((current as i32 + tow).rem_euclid(6), end as i32);

    // let tow_current = ((current as i32) + tow).rem_euclid(6);
    if relative_angle((current as i32 + tow).rem_euclid(6), end_target) >= remaining as i32 {
        tow = towards(current as i32, end_target);
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
    let mut probabilities = [0f32; DIRECTION_NUMBER as usize];
    let hold_probability = rng.next_u64() % 31;

    let total_age = age.iter().sum::<i32>() as f32;
    for (i, a) in age.iter().enumerate() {
        probabilities[i] = *a as f32 / total_age;
    }

    probabilities[current as usize] = hold_probability as f32 / 100.0;

    let chosen = rng.next_u64() % (100 + hold_probability + 1);

    let mut i = 0;
    let mut sum = probabilities[0] * 100.0;

    let mut tow;
    while sum < chosen as f32 && i < 5 {
        i += 1;
        sum += probabilities[i] * 100.0;
    }

    tow = towards(current as i32, i as i32);

    let end_target = calculate_end_target(current as i32, end as i32);

    // let tow_current = ((current as i32) + tow).rem_euclid(6);
    if relative_angle(current as i32, end_target) >= remaining as i32 - 2 {
        tow = towards(current as i32, end_target);
    }

    tow
}

fn towards(current: i32, target: i32) -> i32 {
    if current == target {
        return 0;
    }

    if relative_angle((current + 1).rem_euclid(6), target) < relative_angle((current - 1).rem_euclid(6), target) {
        1
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction::West;
    use crate::{BLOCK_SIZE, DIRECTION_NUMBER, block_at, calculate_anchor, towards, visit_time, wind_at};
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
        println!("{}", seed);

        let block = block_at(seed, 0);
        let block2 = block_at(seed, 1);

        let second_anchor = calculate_anchor(seed, 1);
        println!("Second anchor: {}", second_anchor);
        println!("{:?}", block);

        assert_eq!(block[0], calculate_anchor(seed, 0));
        assert!(
            block[BLOCK_SIZE - 1] == (second_anchor as i32 + 1).rem_euclid(6) as u32
                || block[BLOCK_SIZE - 1] == (second_anchor as i32 - 1).rem_euclid(6) as u32
                || block[BLOCK_SIZE - 1] == second_anchor as i32 as u32
        );
        assert_eq!(block2[0], second_anchor);
    }

    #[test]
    fn wind_at_test() {
        let seed = 1528890;

        // The start direction is 5
        assert_eq!(calculate_anchor(seed, 0), 5);

        // The end direction is 4
        assert_eq!(calculate_anchor(seed, 1), 4);

        let block = block_at(seed, 0);
        assert_eq!(
            [
                5, 0, 1, 2, 3, 3, 2, 3, 4, 3, 2, 1, 0, 5, 0, 5, 0, 5, 4, 3, 2, 2, 1, 0, 5, 4, 3, 2, 3
            ],
            block
        );

        let wind = wind_at(seed, 4);
        assert_eq!(wind, West);
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
        println!("{}", seed);

        let block = block_at(seed, 0);
        let mut visited = [false; DIRECTION_NUMBER as usize];

        println!("{:?}", block);
        // Block is only 30 items
        block.iter().for_each(|x| visited[*x as usize] = true);
        assert_eq!(visited.iter().filter(|x| **x).count(), 6);
        // }
    }
}
