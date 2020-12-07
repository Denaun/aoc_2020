//! Day 7

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, line_ending},
    combinator::{all_consuming, map_res, opt, recognize, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

fn build_containee_map<K1, K2>(data: &[(K1, impl AsRef<[(K2, usize)]>)]) -> HashMap<K2, Vec<K1>>
where
    K1: Eq + Hash + Clone,
    K2: Eq + Hash + Clone,
{
    let mut result = HashMap::<_, Vec<_>>::new();
    for (container, containees) in data.iter() {
        for (containee, _) in containees.as_ref().iter() {
            result
                .entry(containee.clone())
                .or_default()
                .push(container.clone())
        }
    }
    result
}
fn bfs<'a, T>(start: T, data: &'a HashMap<T, Vec<T>>) -> HashSet<&'a T>
where
    T: Eq + Hash,
{
    let mut all_children = HashSet::new();
    let mut to_visit = vec![&start];
    while let Some(next) = to_visit.pop() {
        if let Some(children) = data.get(next) {
            for child in children {
                if !all_children.contains(child) {
                    to_visit.push(child);
                    all_children.insert(child);
                }
            }
        }
    }
    all_children
}
fn build_container_map<K1, K2, V>(data: &[(K1, Vec<(K2, V)>)]) -> HashMap<K1, HashMap<K2, V>>
where
    K1: Eq + Hash + Clone,
    K2: Eq + Hash + Clone,
    V: Clone,
{
    data.iter()
        .map(|(container, containees)| (container.clone(), containees.iter().cloned().collect()))
        .collect()
}
fn count_nesting<T>(end: T, data: &HashMap<T, HashMap<T, usize>>) -> usize
where
    T: Eq + Hash,
{
    let mut sums = HashMap::<&T, usize>::new();
    let mut to_visit = vec![&end];
    while let Some(next) = to_visit.last() {
        let entries = data.get(next).unwrap();
        if entries.iter().all(|(e, _)| sums.contains_key(e)) {
            sums.insert(
                next,
                entries.iter().map(|(e, &mul)| mul * (1 + sums[e])).sum(),
            );
            to_visit.pop().unwrap();
        } else {
            to_visit.extend(entries.iter().filter_map(|(e, _)| {
                if !sums.contains_key(e) {
                    Some(e)
                } else {
                    None
                }
            }));
        }
    }
    sums[&end]
}

fn parse_color(s: &str) -> IResult<&str, &str> {
    recognize(separated_pair(alpha1, char(' '), alpha1))(s)
}
fn parse_quantified_bag(s: &str) -> IResult<&str, (&str, usize)> {
    let (s, num) = map_res(digit1, |s: &str| s.parse())(s)?;
    let (s, _) = char(' ')(s)?;
    let (s, color) = parse_color(s)?;
    let (s, _) = terminated(tag(" bag"), opt(char('s')))(s)?;
    Ok((s, (color, num)))
}
fn parse_line(s: &str) -> IResult<&str, (&str, Vec<(&str, usize)>)> {
    let (s, container) = parse_color(s)?;
    let (s, _) = tag(" bags contain ")(s)?;
    let (s, containees) = terminated(
        alt((
            separated_list1(tag(", "), parse_quantified_bag),
            value(Vec::new(), tag("no other bags")),
        )),
        char('.'),
    )(s)?;
    Ok((s, (container, containees)))
}
fn parse_input(s: &str) -> IResult<&str, Vec<(&str, Vec<(&str, usize)>)>> {
    all_consuming(separated_list1(line_ending, parse_line))(s)
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        bfs(
            "shiny gold",
            &build_containee_map(&parse_input(self).expect("Failed to parse the input").1),
        )
        .len()
    }
    fn part_2(&self) -> usize {
        count_nesting(
            "shiny gold",
            &build_container_map(&parse_input(self).expect("Failed to parse the input").1),
        )
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
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
            ),
            Ok((
                "",
                vec![
                    ("light red", vec![("bright white", 1), ("muted yellow", 2)]),
                    (
                        "dark orange",
                        vec![("bright white", 3), ("muted yellow", 4)]
                    ),
                    ("bright white", vec![("shiny gold", 1)]),
                    ("muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
                    ("shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
                    ("dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
                    ("vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
                    ("faded blue", vec![]),
                    ("dotted black", vec![]),
                ]
            ))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            bfs(
                "shiny gold",
                &build_containee_map(&[
                    ("light red", vec![("bright white", 1), ("muted yellow", 2)]),
                    (
                        "dark orange",
                        vec![("bright white", 3), ("muted yellow", 4)]
                    ),
                    ("bright white", vec![("shiny gold", 1)]),
                    ("muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
                    ("shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
                    ("dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
                    ("vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
                    ("faded blue", vec![]),
                    ("dotted black", vec![]),
                ])
            ),
            ["bright white", "muted yellow", "dark orange", "light red"]
                .iter()
                .collect()
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_7").part_1(), 142);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            count_nesting(
                "shiny gold",
                &build_container_map(&[
                    ("light red", vec![("bright white", 1), ("muted yellow", 2)]),
                    (
                        "dark orange",
                        vec![("bright white", 3), ("muted yellow", 4)]
                    ),
                    ("bright white", vec![("shiny gold", 1)]),
                    ("muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
                    ("shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
                    ("dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
                    ("vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
                    ("faded blue", vec![]),
                    ("dotted black", vec![]),
                ])
            ),
            32
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            count_nesting(
                "shiny gold",
                &build_container_map(
                    &parse_input(
                        "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
                    )
                    .unwrap()
                    .1
                )
            ),
            126
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_7").part_2(), 10219);
    }
}
