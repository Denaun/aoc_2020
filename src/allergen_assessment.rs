//! Day 21

use std::collections::{BTreeMap, HashMap, HashSet};

use itertools::Itertools;

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> String;
}
impl Solution for &str {
    fn part_1(&self) -> usize {
        find_non_allergens(&parsers::input(self).expect("Failed to parse the input")).len()
    }
    fn part_2(&self) -> String {
        find_allergens(&parsers::input(self).expect("Failed to parse the input"))
            .expect("Allergens not found")
            .into_iter()
            .map(|(_, ingredient)| ingredient)
            .join(",")
    }
}

fn get_allergen_candidates<'a, 'b>(
    notes: &[(Vec<&'a str>, Vec<&'b str>)],
) -> HashMap<&'b str, HashSet<&'a str>> {
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
    allergen_candidates
}
fn find_allergens<'a, 'b>(
    notes: &[(Vec<&'a str>, Vec<&'b str>)],
) -> Option<BTreeMap<&'b str, &'a str>> {
    let mut allergen_candidates = get_allergen_candidates(notes);
    let mut allergens = BTreeMap::new();
    // Same idea as Ticket Translation.
    while let Some((&allergen, candidate)) = allergen_candidates.iter().find(|c| c.1.len() == 1) {
        let candidate = *candidate.iter().exactly_one().unwrap();
        for (_, candidates) in allergen_candidates.iter_mut() {
            candidates.remove(&candidate);
        }
        allergens.insert(allergen, candidate);
    }
    if allergen_candidates.iter().any(|(_, c)| !c.is_empty()) {
        None
    } else {
        Some(allergens)
    }
}
fn find_non_allergens<'a>(notes: &[(Vec<&'a str>, Vec<&str>)]) -> Vec<&'a str> {
    let allergen_candidates = get_allergen_candidates(notes);
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

mod parsers {
    use nom::{
        bytes::streaming::tag,
        character::complete::{alpha1, char, line_ending},
        error::Error,
        multi::separated_list1,
        sequence::{delimited, pair},
    };

    use crate::parsers::finished_parser;

    pub fn input(s: &str) -> Result<Vec<(Vec<&str>, Vec<&str>)>, Error<&str>> {
        finished_parser(separated_list1(
            line_ending,
            pair(
                separated_list1(char(' '), alpha1),
                delimited(
                    tag(" (contains "),
                    separated_list1(tag(", "), alpha1),
                    char(')'),
                ),
            ),
        ))(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn example_1() {
        assert_equal(
            find_non_allergens(
                &parsers::input(
                    "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
                )
                .unwrap(),
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

    #[test]
    fn example_2() {
        assert_eq!(
            find_allergens(
                &parsers::input(
                    "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
                )
                .unwrap(),
            )
            .unwrap()
            .iter()
            .map(|(_, ingredient)| ingredient)
            .join(","),
            "mxmxvkd,sqjhc,fvjkl"
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(
            include_str!("inputs/day_21").part_2(),
            "fllssz,kgbzf,zcdcdf,pzmg,kpsdtv,fvvrc,dqbjj,qpxhfp"
        )
    }
}
