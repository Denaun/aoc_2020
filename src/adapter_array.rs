//! Day 10

use itertools::Itertools;
use num_traits::{NumOps, Unsigned};
use std::{collections::HashMap, hash::Hash, iter::once, str::FromStr};

fn adapter_chain<T>(values: &[T]) -> impl Iterator<Item = T>
where
    T: Unsigned + From<u8> + Ord + Clone,
{
    let max = values.iter().max().unwrap().clone();
    once(T::zero())
        .chain(values.iter().cloned().sorted())
        .chain(once(max + T::from(3)))
}
fn diff<T>(it: impl Iterator<Item = T>) -> impl Iterator<Item = T>
where
    T: NumOps + Clone,
{
    it.tuple_windows().map(|(fst, snd)| snd - fst)
}
fn calculate_distribution<T>(it: impl Iterator<Item = T>) -> HashMap<T, usize>
where
    T: Eq + Hash + Clone,
{
    it.fold(HashMap::new(), |mut acc, v| {
        *acc.entry(v).or_default() += 1;
        acc
    })
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let distribution: HashMap<u32, _> = calculate_distribution(diff(adapter_chain(
            &parse_input(self).expect("Failed to parse the input"),
        )));
        distribution[&1] * distribution[&3]
    }
}

fn parse_input<T: FromStr>(text: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    text.lines().map(str::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(
                "\
16
10
15
5
1
11
7
19
6
12
4"
            ),
            Ok(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4])
        );
    }

    #[test]
    fn example_1() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(input.iter().max().unwrap() + 3, 22);
        assert_eq!(
            calculate_distribution(diff(adapter_chain(&input))),
            [(1, 7), (3, 5)]
                .iter()
                .copied()
                .collect::<HashMap<usize, usize>>()
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            calculate_distribution(diff(adapter_chain(&[
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ]))),
            [(1, 22), (3, 10)]
                .iter()
                .copied()
                .collect::<HashMap<usize, usize>>()
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_10").part_1(), 2516);
    }
}
