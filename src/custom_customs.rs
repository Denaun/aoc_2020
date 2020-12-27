//! Day 6

use std::collections::HashSet;

use itertools::Itertools;

fn unique_answers<'a>(group: &'a [&str]) -> impl Iterator<Item = char> + 'a {
    group.iter().flat_map(|answers| answers.chars()).unique()
}
fn common_answers(group: &[&str]) -> Option<HashSet<char>> {
    group
        .iter()
        .map(|answers| answers.chars().collect::<HashSet<_>>())
        .fold1(|mut common, answers| {
            common.retain(|a| answers.contains(a));
            common
        })
}

mod parsers {
    use nom::{
        character::complete::{alpha1, line_ending},
        error::Error,
        multi::separated_list1,
        IResult,
    };

    use crate::parsers::{double_line_ending, finished_parser};

    pub fn input(s: &str) -> Result<Vec<Vec<&str>>, Error<&str>> {
        finished_parser(separated_list1(double_line_ending, group))(s)
    }
    pub fn group(s: &str) -> IResult<&str, Vec<&str>> {
        separated_list1(line_ending, alpha1)(s)
    }
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        parsers::input(self)
            .expect("Failed to parse the input")
            .iter()
            .map(|group| unique_answers(group).count())
            .sum()
    }
    fn part_2(&self) -> usize {
        parsers::input(self)
            .expect("Failed to parse the input")
            .iter()
            .map(|group| {
                if let Some(answers) = common_answers(group) {
                    answers.len()
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example_group() {
        assert_eq!(
            parsers::group(
                "\
abcx
abcy
abcz"
            ),
            Ok(("", vec!["abcx", "abcy", "abcz"]))
        );
    }

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
                "\
abc

a
b
c

ab
ac

a
a
a
a

b"
            ),
            Ok(vec![
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a"; 4],
                vec!["b"]
            ])
        );
    }

    #[test]
    fn example_1() {
        assert_equal(
            unique_answers(&["abcx", "abcy", "abcz"]),
            vec!['a', 'b', 'c', 'x', 'y', 'z'],
        )
    }

    #[test]
    fn example_2() {
        assert_equal(
            [
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a"; 4],
                vec!["b"],
            ]
            .iter()
            .map(|group| unique_answers(group).count()),
            vec![3, 3, 3, 1, 1],
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_6").part_1(), 6249);
    }

    #[test]
    fn example_3() {
        assert_equal(
            [
                vec!["abc"],
                vec!["a", "b", "c"],
                vec!["ab", "ac"],
                vec!["a"; 4],
                vec!["b"],
            ]
            .iter()
            .map(|group| common_answers(group).unwrap().len()),
            vec![3, 0, 1, 1, 1],
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_6").part_2(), 3103);
    }
}
