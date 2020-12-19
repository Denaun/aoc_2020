//! Day 19

use crate::docking_data::parse_integer;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, line_ending},
    combinator::{all_consuming, map},
    error::{Error, ErrorKind},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    Err, IResult,
};
use std::collections::HashMap;

type Parser = Box<dyn FnMut(&str) -> IResult<&str, ()>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct RuleId(usize);
#[derive(Debug, Clone, PartialEq)]
enum Rule {
    Char(char),
    Composite(Vec<Vec<RuleId>>),
}

fn resolve(rules: &HashMap<RuleId, Rule>, root: &RuleId) -> Option<Parser> {
    match rules.get(root)? {
        Rule::Char(c) => {
            let c = *c;
            Some(Box::new(move |s| map(char(c), |_| ())(s)))
        }
        Rule::Composite(union) => {
            let mut union: Vec<Vec<Parser>> = union
                .iter()
                .map(move |sequence| {
                    sequence
                        .iter()
                        .map(|rule| resolve(rules, rule).ok_or(()))
                        .try_collect()
                })
                .try_collect()
                .ok()?;
            Some(Box::new(move |s| {
                'seq: for sequence in &mut union {
                    let mut s1 = s;
                    for rule in sequence {
                        match rule(s1) {
                            Ok((s2, _)) => s1 = s2,
                            _ => {
                                continue 'seq;
                            }
                        }
                    }
                    return Ok((s1, ()));
                }
                Err(Err::Error(Error {
                    input: s,
                    code: ErrorKind::Alt,
                }))
            }))
        }
    }
}

fn parse_input(s: &str) -> IResult<&str, (HashMap<RuleId, Rule>, Vec<&str>)> {
    let (s, rules) = parse_rules(s)?;
    let (s, _) = line_ending(s)?;
    let (s, _) = line_ending(s)?;
    let (s, messages) = separated_list1(line_ending, alpha1)(s)?;
    Ok((s, (rules, messages)))
}
fn parse_rules(s: &str) -> IResult<&str, HashMap<RuleId, Rule>> {
    let parse_id = |s| map(parse_integer, |i| RuleId(i))(s);
    let (s, rules) = separated_list0(
        line_ending,
        separated_pair(
            parse_id,
            tag(": "),
            alt((
                map(delimited(char('"'), anychar, char('"')), |c| Rule::Char(c)),
                map(
                    separated_list1(tag(" | "), separated_list1(char(' '), parse_id)),
                    |seq| Rule::Composite(seq),
                ),
            )),
        ),
    )(s)?;
    Ok((s, rules.into_iter().collect()))
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let (rules, messages) = parse_input(self).expect("Failed to parse the input").1;
        let mut rule_zero =
            all_consuming(resolve(&rules, &RuleId(0)).expect("Failed to resolve rule 0"));
        messages
            .iter()
            .filter(|message| rule_zero(message).is_ok())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(
                "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
            ),
            Ok((
                "",
                (
                    [
                        (
                            RuleId(0),
                            Rule::Composite(vec![vec![RuleId(4), RuleId(1), RuleId(5)]])
                        ),
                        (
                            RuleId(1),
                            Rule::Composite(vec![
                                vec![RuleId(2), RuleId(3)],
                                vec![RuleId(3), RuleId(2)]
                            ])
                        ),
                        (
                            RuleId(2),
                            Rule::Composite(vec![
                                vec![RuleId(4), RuleId(4)],
                                vec![RuleId(5), RuleId(5)]
                            ])
                        ),
                        (
                            RuleId(3),
                            Rule::Composite(vec![
                                vec![RuleId(4), RuleId(5)],
                                vec![RuleId(5), RuleId(4)]
                            ])
                        ),
                        (RuleId(4), Rule::Char('a')),
                        (RuleId(5), Rule::Char('b')),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                    vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
                )
            ))
        );
    }

    #[test]
    fn example_1() {
        let rules = parse_rules(
            "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"",
        )
        .unwrap()
        .1;
        let mut rule_zero = all_consuming(resolve(&rules, &RuleId(0)).unwrap());
        assert!(rule_zero("ababbb").is_ok());
        assert!(!rule_zero("bababa").is_ok());
        assert!(rule_zero("abbbab").is_ok());
        assert!(!rule_zero("aaabbb").is_ok());
        assert!(!rule_zero("aaaabbb").is_ok());
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_19").part_1(), 230);
    }
}
