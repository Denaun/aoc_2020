/// Day 2
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

fn filter_valid<'a>(data: &'a [(Policy, &str)]) -> impl Iterator<Item = &'a str> {
    data.iter().filter_map(|(policy, password)| {
        if policy.is_valid(password) {
            Some(*password)
        } else {
            None
        }
    })
}

#[derive(Debug, PartialEq)]
struct Policy {
    letter: char,
    min: usize,
    max: usize,
}

impl Policy {
    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| *c == self.letter).count();
        count >= self.min && count <= self.max
    }
}

fn policy(input: &str) -> IResult<&str, Policy> {
    let (input, min) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = char('-')(input)?;
    let (input, max) = map_res(digit1, |s: &str| s.parse())(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, letter) = anychar(input)?;

    Ok((input, Policy { letter, min, max }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Policy, &str)>> {
    separated_list0(char('\n'), separated_pair(policy, tag(": "), alpha1))(input)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        filter_valid(&parse_input(self).expect("Failed to parse the input").1).count()
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
                    letter: 'a',
                    min: 1,
                    max: 3,
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
                            letter: 'a',
                            min: 1,
                            max: 3,
                        },
                        "abcde"
                    ),
                    (
                        Policy {
                            letter: 'b',
                            min: 1,
                            max: 3,
                        },
                        "cdefg"
                    ),
                    (
                        Policy {
                            letter: 'c',
                            min: 2,
                            max: 9,
                        },
                        "ccccccccc"
                    ),
                ]
            ))
        )
    }

    #[test]
    fn example_valid() {
        assert!(Policy {
            letter: 'a',
            min: 1,
            max: 3,
        }
        .is_valid("abcde"));
        assert!(!Policy {
            letter: 'b',
            min: 1,
            max: 3,
        }
        .is_valid("cdefg"))
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
                .1
            )
            .collect::<Vec<_>>(),
            vec!["abcde", "ccccccccc"]
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_2").part_1(), 517);
    }
}
