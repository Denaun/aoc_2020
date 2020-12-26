//! Day 16

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{char, digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{cmp::PartialOrd, collections::HashSet, str::FromStr};

use crate::docking_data::parse_integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Range<T> {
    min: T,
    max: T,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RangeUnion<T> {
    first: Range<T>,
    second: Range<T>,
}

fn invalid_fields<'a, T: PartialOrd>(
    fields: &'a [T],
    rules: &'a [RangeUnion<T>],
) -> impl Iterator<Item = &'a T> {
    fields
        .iter()
        .filter(move |field| !rules.into_iter().any(|rule| rule.is_valid(field)))
}
fn find_fields<'a, T: PartialOrd>(
    tickets: &[Vec<T>],
    rules: &[RangeUnion<T>],
) -> Option<Vec<usize>> {
    let valid_tickets: Vec<_> = tickets
        .iter()
        .filter(|ticket| invalid_fields(ticket, rules).next().is_none())
        .collect();
    let n_fields = valid_tickets.first().map(|t| t.len()).unwrap_or(0);
    assert!(valid_tickets.iter().all(|ticket| ticket.len() == n_fields));
    let mut valid_fields_per_rule = rules
        .iter()
        .map(|rule| {
            (0..n_fields)
                .filter(|&field_ix| {
                    valid_tickets
                        .iter()
                        .all(|ticket| rule.is_valid(&ticket[field_ix]))
                })
                .collect::<HashSet<_>>()
        })
        .collect_vec();
    let mut field_indices = vec![0; valid_fields_per_rule.len()];
    // Iteratively remove rules that identify exactly one field from the
    // candidates, until we either find a solution or end up in an undecidable
    // state.
    while let Some((rule_ix, field_ix)) = valid_fields_per_rule
        .iter()
        .enumerate()
        .find(|c| c.1.len() == 1)
    {
        let field_ix = *field_ix.iter().exactly_one().unwrap();
        for candidates in valid_fields_per_rule.iter_mut() {
            candidates.remove(&field_ix);
        }
        field_indices[rule_ix] = field_ix;
    }
    if valid_fields_per_rule.iter().any(|c| !c.is_empty()) {
        None
    } else {
        Some(field_indices)
    }
}

impl<T: PartialOrd> Range<T> {
    fn is_valid(&self, v: &T) -> bool {
        &self.min <= v && v <= &self.max
    }
}
impl<T: PartialOrd> RangeUnion<T> {
    fn is_valid(&self, v: &T) -> bool {
        self.first.is_valid(v) || self.second.is_valid(v)
    }
}

fn parse_input<T: FromStr>(
    s: &str,
) -> IResult<&str, (Vec<(&str, RangeUnion<T>)>, Vec<T>, Vec<Vec<T>>)> {
    let (s, rules) = terminated(separated_list1(line_ending, parse_rule), line_ending)(s)?;
    let (s, _) = line_ending(s)?;
    let (s, _) = terminated(tag("your ticket:"), line_ending)(s)?;
    let (s, yours) = terminated(separated_list1(char(','), parse_integer), line_ending)(s)?;
    let (s, _) = line_ending(s)?;
    let (s, _) = terminated(tag("nearby tickets:"), line_ending)(s)?;
    let (s, nearby) = separated_list1(line_ending, separated_list1(char(','), parse_integer))(s)?;
    Ok((s, (rules, yours, nearby)))
}

fn parse_rule<T: FromStr>(s: &str) -> IResult<&str, (&str, RangeUnion<T>)> {
    separated_pair(take_till(|c| c == ':'), tag(": "), parse_range_union)(s)
}
fn parse_range_union<T: FromStr>(s: &str) -> IResult<&str, RangeUnion<T>> {
    let (s, (first, second)) = separated_pair(parse_range, tag(" or "), parse_range)(s)?;
    Ok((s, RangeUnion { first, second }))
}
fn parse_range<T: FromStr>(s: &str) -> IResult<&str, Range<T>> {
    let (s, (min, max)) = separated_pair(parse_integer, char('-'), parse_integer)(s)?;
    Ok((s, Range { min, max }))
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let (rules, _, tickets) = parse_input(self).expect("Failed to parse the input").1;
        let rules = rules.into_iter().map(|(_, rule)| rule).collect::<Vec<_>>();
        tickets
            .iter()
            .flat_map(|ticket| invalid_fields(ticket, &rules))
            .sum()
    }
    fn part_2(&self) -> usize {
        let (named_rules, your_ticket, tickets) = parse_input::<usize>(self)
            .expect("Failed to parse the input")
            .1;
        let rules = named_rules
            .iter()
            .map(|(_, rule)| *rule)
            .collect::<Vec<_>>();
        find_fields(&tickets, &rules)
            .expect("Field mapping not found")
            .iter()
            .enumerate()
            .filter_map(|(rule_ix, &field_ix)| {
                if named_rules[rule_ix].0.starts_with("departure") {
                    Some(your_ticket[field_ix])
                } else {
                    None
                }
            })
            .product()
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(
                "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            ),
            Ok((
                "",
                (
                    vec![
                        (
                            "class",
                            RangeUnion {
                                first: Range { min: 1, max: 3 },
                                second: Range { min: 5, max: 7 }
                            }
                        ),
                        (
                            "row",
                            RangeUnion {
                                first: Range { min: 6, max: 11 },
                                second: Range { min: 33, max: 44 }
                            }
                        ),
                        (
                            "seat",
                            RangeUnion {
                                first: Range { min: 13, max: 40 },
                                second: Range { min: 45, max: 50 }
                            }
                        ),
                    ],
                    vec![7, 1, 14],
                    vec![
                        vec![7, 3, 47],
                        vec![40, 4, 50],
                        vec![55, 2, 20],
                        vec![38, 6, 12],
                    ]
                )
            ))
        );
    }

    #[test]
    fn example_1() {
        let rules = &[
            RangeUnion {
                first: Range { min: 1, max: 3 },
                second: Range { min: 5, max: 7 },
            },
            RangeUnion {
                first: Range { min: 6, max: 11 },
                second: Range { min: 33, max: 44 },
            },
            RangeUnion {
                first: Range { min: 13, max: 40 },
                second: Range { min: 45, max: 50 },
            },
        ];
        assert_equal(invalid_fields(&[7, 3, 47], rules), &[]);
        assert_equal(invalid_fields(&[40, 4, 50], rules), &[4]);
        assert_equal(invalid_fields(&[55, 2, 20], rules), &[55]);
        assert_equal(invalid_fields(&[38, 6, 12], rules), &[12]);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_16").part_1(), 23115);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            find_fields(
                &[vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]],
                &[
                    RangeUnion {
                        first: Range { min: 0, max: 1 },
                        second: Range { min: 4, max: 19 },
                    },
                    RangeUnion {
                        first: Range { min: 0, max: 5 },
                        second: Range { min: 8, max: 19 },
                    },
                    RangeUnion {
                        first: Range { min: 0, max: 13 },
                        second: Range { min: 16, max: 19 },
                    },
                ]
            ),
            Some(vec![1, 0, 2])
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_16").part_2(), 239_727_793_813);
    }
}
