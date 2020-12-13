//! Day 13

use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending},
    combinator::{all_consuming, map, map_res, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::{
    ops::{Rem, Sub},
    str::FromStr,
};

fn wait_time<T>(from: T, period: T) -> T
where
    T: Sub<Output = T> + Rem<Output = T> + Copy,
{
    period - (from % period)
}

pub fn least_multiple_above<'a>(
    candidates: impl IntoIterator<Item = &'a u32>,
    threshold: u32,
) -> Option<&'a u32> {
    candidates
        .into_iter()
        .min_by_key(|v| wait_time(threshold, **v))
}

fn parse_integer<T: FromStr>(s: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse())(s)
}

fn parse_input<T: FromStr + Clone>(s: &str) -> IResult<&str, (T, Vec<Option<T>>)> {
    all_consuming(separated_pair(
        parse_integer,
        line_ending,
        separated_list1(
            char(','),
            alt((map(parse_integer, |i| Some(i)), value(None, char('x')))),
        ),
    ))(s)
}

trait Solution {
    fn part_1(&self) -> u32;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        let (threshold, candidates) = parse_input(self).expect("Failed to parse the input").1;
        let id = least_multiple_above(candidates.iter().flatten(), threshold).expect("No ID found");
        let remaining = wait_time(threshold, *id);
        id * remaining
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
939
7,13,x,x,59,x,31,19"
            ),
            Ok((
                "",
                (
                    939,
                    vec![
                        Some(7),
                        Some(13),
                        None,
                        None,
                        Some(59),
                        None,
                        Some(31),
                        Some(19)
                    ]
                )
            ))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(least_multiple_above(&[7, 13, 59, 31, 19], 939), Some(&59));
        assert_eq!(wait_time(939, 59), 5);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_13").part_1(), 296);
    }
}
