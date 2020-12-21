//! Day 21

use nom::{
    bytes::streaming::tag,
    character::complete::{alpha1, char, line_ending},
    multi::separated_list1,
    sequence::{delimited, pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

fn find_non_allergens<'a>(notes: &[(Vec<&'a str>, Vec<&str>)]) -> Vec<&'a str> {
    let mut allergen_candidates = HashMap::<&str, HashSet<&str>>::new();
    for (ingredients, allergens) in notes {
        for allergen in allergens {
            allergen_candidates
                .entry(allergen)
                .and_modify(|candidates| {
                    candidates.retain(|c| ingredients.contains(c));
                })
                .or_insert(ingredients.iter().copied().collect::<HashSet<_>>());
        }
    }
    notes
        .iter()
        .flat_map(|(ingredients, _)| {
            ingredients.iter().copied().filter(|ingredient| {
                !allergen_candidates
                    .iter()
                    .any(|(_, candidates)| candidates.contains(ingredient))
            })
        })
        .collect()
}

fn parse_input(s: &str) -> IResult<&str, Vec<(Vec<&str>, Vec<&str>)>> {
    separated_list1(
        line_ending,
        pair(
            separated_list1(char(' '), alpha1),
            delimited(
                tag(" (contains "),
                separated_list1(tag(", "), alpha1),
                char(')'),
            ),
        ),
    )(s)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for &str {
    fn part_1(&self) -> usize {
        find_non_allergens(&parse_input(self).expect("Failed to parse the input").1).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::{assert_equal, Itertools};

    #[test]
    fn example_1() {
        assert_equal(
            find_non_allergens(
                &parse_input(
                    "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
                )
                .unwrap()
                .1,
            )
            .iter()
            .sorted(),
            &["kfcds", "nhms", "sbzzf", "sbzzf", "trh"],
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_21").part_1(), 1885)
    }
}
