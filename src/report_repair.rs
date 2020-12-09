//! Day 1

use num_traits::{CheckedSub, Zero};
use std::{ops::Mul, str::FromStr};

pub fn find_sum<T: Zero + CheckedSub + Copy>(values: &[T], sum: T, n: usize) -> Option<Vec<T>> {
    if n == 0 {
        return if sum.is_zero() {
            Some(Vec::new())
        } else {
            None
        };
    }

    values.iter().enumerate().skip(n - 1).find_map(|(ix, v)| {
        if let Some(sum) = sum.checked_sub(v) {
            if let Some(mut values) = find_sum(&values[..ix], sum, n - 1) {
                values.push(*v);
                Some(values)
            } else {
                None
            }
        } else {
            None
        }
    })
}

fn parse_input<T: FromStr>(text: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    text.lines().map(str::parse).collect()
}

trait Solution {
    fn part_1(&self) -> u32;
    fn part_2(&self) -> u32;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        find_sum(
            &parse_input(self).expect("Failed to parse the input"),
            2020,
            2,
        )
        .expect("Solution not found")
        .iter()
        .fold(1, Mul::mul)
    }
    fn part_2(&self) -> u32 {
        find_sum(
            &parse_input(self).expect("Failed to parse the input"),
            2020,
            3,
        )
        .expect("Solution not found")
        .iter()
        .fold(1, Mul::mul)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(
                "\
1721
979
366
299
675
1456"
            ),
            Ok(vec![1721, 979, 366, 299, 675, 1456])
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            find_sum(&[1721, 979, 366, 299, 675, 1456], 2020, 2),
            Some(vec![1721, 299])
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_1").part_1(), 776064);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            find_sum(&[1721, 979, 366, 299, 675, 1456], 2020, 3),
            Some(vec![979, 366, 675])
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_1").part_2(), 6964490);
    }
}
