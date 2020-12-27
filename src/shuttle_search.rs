//! Day 13

use std::ops::{Rem, Sub};

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

fn sparse_offsets(values: impl IntoIterator<Item = Option<i64>> + Clone) -> (Vec<i64>, Vec<i64>) {
    values
        .into_iter()
        .enumerate()
        .filter_map(|(i, v)| v.map(|v| (i as i64, v)))
        .unzip()
}
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}
fn chinese_remainder_inv(inv_residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let residues: Vec<_> = inv_residues
        .iter()
        .zip(modulii)
        .map(|(residue, modulus)| modulus - residue)
        .collect();
    chinese_remainder(&residues, modulii)
}

mod parsers {
    use std::str::FromStr;

    use nom::{
        branch::alt,
        character::complete::{char, line_ending},
        combinator::{map, value},
        error::Error,
        multi::separated_list1,
        sequence::separated_pair,
    };

    use crate::parsers::{finished_parser, integer};

    pub fn input<T: FromStr + Clone>(s: &str) -> Result<(T, Vec<Option<T>>), Error<&str>> {
        finished_parser(separated_pair(
            integer,
            line_ending,
            separated_list1(char(','), alt((map(integer, Some), value(None, char('x'))))),
        ))(s)
    }
}

trait Solution {
    fn part_1(&self) -> u32;
    fn part_2(&self) -> i64;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        let (threshold, candidates) = parsers::input(self).expect("Failed to parse the input");
        let id = least_multiple_above(candidates.iter().flatten(), threshold).expect("No ID found");
        let remaining = wait_time(threshold, *id);
        id * remaining
    }
    fn part_2(&self) -> i64 {
        let (offsets, ids) =
            sparse_offsets(parsers::input(self).expect("Failed to parse the input").1);
        chinese_remainder_inv(&offsets, &ids).expect("IDs not pairwise coprime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
                "\
939
7,13,x,x,59,x,31,19"
            ),
            Ok((
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

    #[test]
    fn example_2() {
        assert_eq!(
            chinese_remainder_inv(&[0, 1, 4, 6, 7], &[7, 13, 59, 31, 19]),
            Some(1_068_781)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(chinese_remainder_inv(&[0, 2, 3], &[17, 13, 19]), Some(3417));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            chinese_remainder_inv(&[0, 1, 2, 3], &[67, 7, 59, 61]),
            Some(754_018)
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            chinese_remainder_inv(&[0, 2, 3, 4], &[67, 7, 59, 61]),
            Some(779_210)
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            chinese_remainder_inv(&[0, 1, 3, 4], &[67, 7, 59, 61]),
            Some(1_261_476)
        );
    }

    #[test]
    fn example_7() {
        assert_eq!(
            chinese_remainder_inv(&[0, 1, 2, 3], &[1789, 37, 47, 1889]),
            Some(1_202_161_486)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_13").part_2(), 535_296_695_251_210);
    }
}
