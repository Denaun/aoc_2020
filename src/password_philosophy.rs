//! Day 2

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

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
struct Policy {
    range: [usize; 2],
    letter: char,
}

fn policy(input: &str) -> IResult<&str, Policy> {
    let (input, first) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, second) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, letter) = anychar(input)?;

    Ok((
        input,
        Policy {
            range: [first, second],
            letter,
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Policy, &str)>> {
    separated_list0(char('\n'), separated_pair(policy, tag(": "), alpha1))(input)
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        filter_valid(
            &parse_input(self).expect("Failed to parse the input").1,
            part_1_rule,
        )
        .count()
    }
    fn part_2(&self) -> usize {
        filter_valid(
            &parse_input(self).expect("Failed to parse the input").1,
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
            policy("1-3 a"),
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
            parse_input(
                "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            ),
            Ok((
                "",
                vec![
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
                ]
            ))
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
                &parse_input(
                    "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
                )
                .unwrap()
                .1,
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
                &parse_input(
                    "\
    1-3 a: abcde
    1-3 b: cdefg
    2-9 c: ccccccccc"
                )
                .unwrap()
                .1,
                part_1_rule
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
