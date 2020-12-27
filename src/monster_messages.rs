//! Day 19

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    character::complete::char,
    combinator::{all_consuming, map},
    error::{Error, ErrorKind},
    Err, IResult,
};

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let (rules, messages) = parsers::input(self).expect("Failed to parse the input");
        let mut rule_zero =
            all_consuming(resolve(&rules, &RuleId(0)).expect("Failed to resolve rule 0"));
        messages
            .into_iter()
            .filter(|message| rule_zero(message).is_ok())
            .count()
    }
    fn part_2(&self) -> usize {
        let (rules, messages) = parsers::input(self).expect("Failed to parse the input");
        let rules = apply_part_2(rules);
        let mut rule_zero =
            all_consuming(resolve(&rules, &RuleId(0)).expect("Failed to resolve rule 0"));
        messages
            .into_iter()
            .filter(|message| rule_zero(message).is_ok())
            .count()
    }
}

type Parser = Box<dyn FnMut(&str) -> IResult<&str, ()>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RuleId(usize);
#[derive(Debug, Clone, PartialEq)]
pub enum Rule {
    Char(char),
    Composite(Vec<Vec<RuleId>>),
}

fn resolve(rules: &HashMap<RuleId, Rule>, root: &RuleId) -> Option<Parser> {
    // Special case for part 2.
    if root == &RuleId(0)
        && rules.get(&RuleId(0)) == Some(&Rule::Composite(vec![vec![RuleId(8), RuleId(11)]]))
        && rules.get(&RuleId(8))
            == Some(&Rule::Composite(vec![
                vec![RuleId(42)],
                vec![RuleId(42), RuleId(8)],
            ]))
        && rules.get(&RuleId(11))
            == Some(&Rule::Composite(vec![
                vec![RuleId(42), RuleId(31)],
                vec![RuleId(42), RuleId(11), RuleId(31)],
            ]))
    {
        let mut forty_two = resolve(rules, &RuleId(42))?;
        let mut thirty_one = resolve(rules, &RuleId(31))?;
        return Some(Box::new(move |s| {
            // Rule 11 matches 42 N > 0 times and 31 N times. Rule 8 matches 42
            // M > 0 times in a non-greedy way, which means that it fills any
            // difference between 42 and 31 above, as long as we have at least 2
            // matches of 42.
            let (s, _) = forty_two(s)?;
            let (mut s, _) = forty_two(s)?;
            let mut count = 0;
            while let Ok((s1, _)) = forty_two(s) {
                s = s1;
                count += 1;
            }
            let (mut s, _) = thirty_one(s)?;
            for _ in 0..count {
                if let Ok((s1, _)) = thirty_one(s) {
                    s = s1;
                } else {
                    break;
                }
            }
            Ok((s, ()))
        }));
    }
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
fn apply_part_2(mut rules: HashMap<RuleId, Rule>) -> HashMap<RuleId, Rule> {
    rules.insert(
        RuleId(8),
        Rule::Composite(vec![vec![RuleId(42)], vec![RuleId(42), RuleId(8)]]),
    );
    rules.insert(
        RuleId(11),
        Rule::Composite(vec![
            vec![RuleId(42), RuleId(31)],
            vec![RuleId(42), RuleId(11), RuleId(31)],
        ]),
    );
    rules
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, anychar, char, line_ending},
        combinator::map,
        error::Error,
        multi::{separated_list0, separated_list1},
        sequence::{delimited, separated_pair},
        IResult,
    };

    use crate::parsers::*;

    use super::*;

    pub fn input(s: &str) -> Result<(HashMap<RuleId, Rule>, Vec<&str>), Error<&str>> {
        finished_parser(separated_pair(
            rules,
            double_line_ending,
            separated_list1(line_ending, alpha1),
        ))(s)
    }
    pub fn rules(s: &str) -> IResult<&str, HashMap<RuleId, Rule>> {
        let (s, rules) = separated_list0(
            line_ending,
            separated_pair(
                id,
                tag(": "),
                alt((
                    map(delimited(char('"'), anychar, char('"')), |c| Rule::Char(c)),
                    map(
                        separated_list1(tag(" | "), separated_list1(char(' '), id)),
                        |seq| Rule::Composite(seq),
                    ),
                )),
            ),
        )(s)?;
        Ok((s, rules.into_iter().collect()))
    }
    fn id(s: &str) -> IResult<&str, RuleId> {
        map(integer, |i| RuleId(i))(s)
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
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
            ))
        );
    }

    #[test]
    fn example_1() {
        let rules = parsers::rules(
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

    #[test]
    fn example_2() {
        let (rules, messages) = parsers::input(
            "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        )
        .unwrap();
        let mut rule_zero = all_consuming(resolve(&rules, &RuleId(0)).unwrap());
        assert_equal(
            messages.iter().filter(|message| rule_zero(message).is_ok()),
            &["bbabbbbaabaabba", "ababaaaaaabaaab", "ababaaaaabbbaba"],
        );
        let rules = apply_part_2(rules);
        let mut rule_zero = all_consuming(resolve(&rules, &RuleId(0)).unwrap());
        assert_equal(
            messages.iter().filter(|message| rule_zero(message).is_ok()),
            &[
                "bbabbbbaabaabba",
                "babbbbaabbbbbabbbbbbaabaaabaaa",
                "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
                "bbbbbbbaaaabbbbaaabbabaaa",
                "bbbababbbbaaaaaaaabbababaaababaabab",
                "ababaaaaaabaaab",
                "ababaaaaabbbaba",
                "baabbaaaabbaaaababbaababb",
                "abbbbabbbbaaaababbbbbbaaaababb",
                "aaaaabbaabaaaaababaa",
                "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
                "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            ],
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_19").part_2(), 341);
    }
}
