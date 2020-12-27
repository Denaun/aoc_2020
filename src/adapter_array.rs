//! Day 10

use std::{collections::HashMap, hash::Hash, iter::once};

use itertools::Itertools;
use num_traits::{NumOps, Unsigned};

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let distribution: HashMap<u32, _> = calculate_distribution(diff(adapter_chain(
            &parsers::input(self).expect("Failed to parse the input"),
        )));
        distribution[&1] * distribution[&3]
    }
    fn part_2(&self) -> usize {
        let chain: Vec<usize> =
            adapter_chain(&parsers::input(self).expect("Failed to parse the input")).collect_vec();
        count_arrangements(&chain)
    }
}

const MAX_OFFSET: u8 = 3;

fn adapter_chain<T>(values: &[T]) -> impl Iterator<Item = T>
where
    T: Unsigned + From<u8> + Ord + Clone,
{
    let max = values.iter().max().unwrap().clone();
    once(T::zero())
        .chain(values.iter().cloned().sorted())
        .chain(once(max + T::from(MAX_OFFSET)))
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
fn count_arrangements<T>(values: &[T]) -> usize
where
    T: NumOps + From<u8> + Ord + Clone,
{
    let mut path_counts = vec![0usize; values.len()];
    *path_counts.last_mut().unwrap() = 1;
    for ix in (0..values.len() - 1).rev() {
        path_counts[ix] = (1..=MAX_OFFSET as usize)
            .map(|offset| ix + offset)
            .filter(|&neighbor| {
                neighbor < values.len()
                    && values[neighbor] <= values[ix].clone() + T::from(MAX_OFFSET)
            })
            .map(|neighbor| path_counts[neighbor])
            .sum();
    }
    *path_counts.first().unwrap()
}

mod parsers {
    pub use crate::parsers::number_list as input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
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

    #[test]
    fn example_3() {
        let chain: Vec<u32> = adapter_chain(&[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]).collect_vec();
        assert_eq!(count_arrangements(&chain), 8);
    }

    #[test]
    fn example_4() {
        let chain: Vec<u32> = adapter_chain(&[
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ])
        .collect_vec();
        assert_eq!(count_arrangements(&chain), 19208);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_10").part_2(), 296_196_766_695_424);
    }
}
