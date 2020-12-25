//! Day 25

use itertools::iterate;
use nom::{character::complete::line_ending, sequence::separated_pair, IResult};

use crate::docking_data::parse_integer;

const SUBJECT_NUMBER: u64 = 7;
const INITIAL_KEY: u64 = 1;
const LOOP_REMINDER: u64 = 20201227;

fn transformations(subject_number: u64) -> impl Iterator<Item = u64> {
    iterate(INITIAL_KEY, move |k| (k * subject_number) % LOOP_REMINDER)
}
fn find_loop_size(public_key: u64) -> usize {
    transformations(SUBJECT_NUMBER)
        .take_while(|k| k != &public_key)
        .enumerate()
        .last()
        .map(|(i, _)| i + 1)
        .unwrap()
}

fn parse_input(s: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(parse_integer, line_ending, parse_integer)(s)
}

trait Solution {
    fn part_1(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        let (card, door) = parse_input(self).expect("Failed to parse the input").1;
        let loop_size = find_loop_size(card);
        transformations(door).nth(loop_size).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_loop_size(5764801), 8);
        assert_eq!(find_loop_size(17807724), 11);
        assert_eq!(transformations(5764801).nth(11), Some(14897079));
        assert_eq!(transformations(17807724).nth(8), Some(14897079));
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_25").part_1(), 2947148)
    }
}
