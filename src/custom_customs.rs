//! Day 6

use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::pair,
    IResult,
};

fn unique_answers<'a>(group: &'a [&str]) -> impl Iterator<Item = char> + 'a {
    group.iter().flat_map(|answers| answers.chars()).unique()
}

fn parse_group(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1)(s)
}
fn parse_input(s: &str) -> IResult<&str, Vec<Vec<&str>>> {
    separated_list0(pair(line_ending, line_ending), parse_group)(s)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .iter()
            .map(|group| unique_answers(group).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_group() {
        assert_eq!(
            parse_group(
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
            parse_input(
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
            Ok((
                "",
                vec![
                    vec!["abc"],
                    vec!["a", "b", "c"],
                    vec!["ab", "ac"],
                    vec!["a"; 4],
                    vec!["b"]
                ]
            ))
        );
    }

    #[test]
    fn example_1() {
        itertools::assert_equal(
            unique_answers(&["abcx", "abcy", "abcz"]),
            vec!['a', 'b', 'c', 'x', 'y', 'z'],
        )
    }

    #[test]
    fn example_2() {
        itertools::assert_equal(
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
}
