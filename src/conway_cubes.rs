//! Day 17

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

type Coordinate = Vec<i64>;

#[derive(Debug, Clone, PartialEq)]
pub struct PocketDimension {
    dimensions: usize,
    active_cubes: HashSet<Coordinate>,
}
impl PocketDimension {
    pub fn evolve(&mut self) {
        let mut to_deactivate = Vec::new();
        let mut to_activate = HashMap::<Coordinate, usize>::new();
        for coord in self.active_cubes.iter() {
            let neighbors = self.neighbors(coord);
            let n_active = neighbors
                .iter()
                .filter(|&n| self.active_cubes.contains(n))
                .count();
            if n_active != 2 && n_active != 3 {
                to_deactivate.push(coord.clone());
            }
            for inactive in neighbors {
                if !self.active_cubes.contains(&inactive) {
                    *to_activate.entry(inactive).or_default() += 1;
                }
            }
        }
        for coord in to_deactivate {
            self.active_cubes.remove(&coord);
        }
        for (coord, n_active) in to_activate {
            if n_active == 3 {
                self.active_cubes.insert(coord);
            }
        }
    }

    fn neighbors(&self, coord: &Coordinate) -> Vec<Coordinate> {
        repeat(&[-1, 0, 1])
            .take(self.dimensions)
            .multi_cartesian_product()
            .map(|off| coord.iter().zip(off).map(|(c, o)| c + o).collect_vec())
            .filter(|c| c != coord)
            .collect()
    }
}

mod parsers {
    use nom::error::Error;

    use crate::parsers::{bw_image, finished_parser};

    use super::PocketDimension;

    pub fn input(dimensions: usize) -> impl FnMut(&str) -> Result<PocketDimension, Error<&str>> {
        move |s| {
            finished_parser(bw_image)(s).map(|cubes| PocketDimension {
                dimensions,
                active_cubes: cubes
                    .into_iter()
                    .enumerate()
                    .flat_map(|(x, row)| {
                        row.into_iter().enumerate().filter_map(move |(y, active)| {
                            if active {
                                let mut coord = vec![0; dimensions];
                                coord[0] = x as i64;
                                coord[1] = y as i64;
                                Some(coord)
                            } else {
                                None
                            }
                        })
                    })
                    .collect(),
            })
        }
    }
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let mut dimension = parsers::input(3)(self).expect("Failed to parse the input");
        for _ in 0..6 {
            dimension.evolve();
        }
        dimension.active_cubes.len()
    }
    fn part_2(&self) -> usize {
        let mut dimension = parsers::input(4)(self).expect("Failed to parse the input");
        for _ in 0..6 {
            dimension.evolve();
        }
        dimension.active_cubes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(3)(
                "\
.#.
..#
###"
            ),
            Ok(PocketDimension {
                dimensions: 3,
                active_cubes: [
                    vec![0, 1, 0],
                    vec![1, 2, 0],
                    vec![2, 0, 0],
                    vec![2, 1, 0],
                    vec![2, 2, 0]
                ]
                .iter()
                .cloned()
                .collect()
            })
        );
    }

    #[test]
    fn example_1() {
        let mut dimension = parsers::input(3)(
            "\
.#.
..#
###",
        )
        .unwrap();
        for _ in 0..6 {
            dimension.evolve();
        }
        assert_eq!(dimension.active_cubes.len(), 112);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_17").part_1(), 301);
    }

    #[test]
    fn example_2() {
        let mut dimension = parsers::input(4)(
            "\
.#.
..#
###",
        )
        .unwrap();
        for _ in 0..6 {
            dimension.evolve();
        }
        assert_eq!(dimension.active_cubes.len(), 848);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_17").part_2(), 2424);
    }
}
