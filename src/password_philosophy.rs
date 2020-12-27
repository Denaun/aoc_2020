//! Day 2

fn filter_valid<'a>(
    data: &'a [(Policy, &str)],
    is_valid: impl Fn(&Policy, &str) -> bool,
) -> impl Iterator<Item = &'a str> {
    data.iter().filter_map(move |(policy, password)| {
        if is_valid(policy, password) {
            Some(*password)
        } else {
            None
        }
    })
}

fn part_1_rule(policy: &Policy, password: &str) -> bool {
    let count = password.chars().filter(|c| *c == policy.letter).count();
    count >= policy.range[0] && count <= policy.range[1]
}

fn part_2_rule(policy: &Policy, password: &str) -> bool {
    password
        .char_indices()
        .filter(|(ix, c)| policy.range.contains(&(ix + 1)) && *c == policy.letter)
        .count()
        == 1
}

#[derive(Debug, PartialEq)]
pub struct Policy {
    range: [usize; 2],
    letter: char,
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, char, line_ending},
        error::Error,
        multi::separated_list0,
        sequence::separated_pair,
        IResult,
    };

    use crate::parsers::{finished_parser, integer};

    use super::Policy;

    pub fn input(s: &str) -> Result<Vec<(Policy, &str)>, Error<&str>> {
        finished_parser(separated_list0(
            line_ending,
            separated_pair(policy, tag(": "), alpha1),
        ))(s)
    }

    pub fn policy(s: &str) -> IResult<&str, Policy> {
        let (s, first) = integer(s)?;
        let (s, _) = char('-')(s)?;
        let (s, second) = integer(s)?;
        let (s, _) = char(' ')(s)?;
        let (s, letter) = anychar(s)?;

        Ok((
            s,
            Policy {
                range: [first, second],
                letter,
            },
        ))
    }
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        filter_valid(
            &parsers::input(self).expect("Failed to parse the input"),
            part_1_rule,
        )
        .count()
    }
    fn part_2(&self) -> usize {
        filter_valid(
            &parsers::input(self).expect("Failed to parse the input"),
            part_2_rule,
        )
        .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_policy() {
        assert_eq!(
            parsers::policy("1-3 a"),
            Ok((
                "",
                Policy {
                    range: [1, 3],
                    letter: 'a',
                }
            ))
        )
    }

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
                "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            ),
            Ok(vec![
                (
                    Policy {
                        range: [1, 3],
                        letter: 'a',
                    },
                    "abcde"
                ),
                (
                    Policy {
                        range: [1, 3],
                        letter: 'b',
                    },
                    "cdefg"
                ),
                (
                    Policy {
                        range: [2, 9],
                        letter: 'c',
                    },
                    "ccccccccc"
                ),
            ])
        )
    }

    #[test]
    fn example_part_1_rule() {
        assert!(part_1_rule(
            &Policy {
                range: [1, 3],
                letter: 'a',
            },
            "abcde"
        ));
        assert!(!part_1_rule(
            &Policy {
                range: [1, 3],
                letter: 'b',
            },
            "cdefg"
        ))
    }

    #[test]
    fn example_1() {
        assert_eq!(
            filter_valid(
                &parsers::input(
                    "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
                )
                .unwrap(),
                part_1_rule
            )
            .collect::<Vec<_>>(),
            vec!["abcde", "ccccccccc"]
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_2").part_1(), 517);
    }

    #[test]
    fn example_part_2_rule() {
        assert!(part_2_rule(
            &Policy {
                range: [1, 3],
                letter: 'a',
            },
            "abcde"
        ));
        assert!(!part_2_rule(
            &Policy {
                range: [2, 9],
                letter: 'c',
            },
            "ccccccccc"
        ))
    }

    #[test]
    fn example_2() {
        assert_eq!(
            filter_valid(
                &parsers::input(
                    "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
                )
                .unwrap(),
                part_2_rule
            )
            .collect::<Vec<_>>(),
            vec!["abcde"]
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_2").part_2(), 284);
    }
}
