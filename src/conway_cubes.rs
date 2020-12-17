//! Day 17

use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, value},
    multi::{many0, separated_list0},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    iter::repeat,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(i64, i64, i64);
#[derive(Debug, Clone, PartialEq)]
struct PocketDimension {
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
                .filter(|n| self.active_cubes.contains(n))
                .count();
            if n_active != 2 && n_active != 3 {
                to_deactivate.push(*coord);
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
            .take(3)
            .multi_cartesian_product()
            .map(|off| Coordinate(coord.0 + off[0], coord.1 + off[1], coord.2 + off[2]))
            .filter(|c| c != coord)
            .collect()
    }
}

fn parse_input(s: &str) -> IResult<&str, PocketDimension> {
    let (s, cubes) = all_consuming(separated_list0(
        line_ending,
        many0(alt((value(false, char('.')), value(true, char('#'))))),
    ))(s)?;

    Ok((
        s,
        PocketDimension {
            active_cubes: cubes
                .into_iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.into_iter().enumerate().filter_map(move |(y, active)| {
                        if active {
                            Some(Coordinate(x as i64, y as i64, 0i64))
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        },
    ))
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let mut dimension = parse_input(self).expect("Failed to parse the input").1;
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
            parse_input(
                "\
.#.
..#
###"
            ),
            Ok((
                "",
                PocketDimension {
                    active_cubes: [(0, 1, 0), (1, 2, 0), (2, 0, 0), (2, 1, 0), (2, 2, 0)]
                        .iter()
                        .map(|p| Coordinate(p.0, p.1, p.2))
                        .collect()
                }
            ))
        );
    }

    #[test]
    fn example_1() {
        let mut dimension = PocketDimension {
            active_cubes: [(0, 1, 0), (1, 2, 0), (2, 0, 0), (2, 1, 0), (2, 2, 0)]
                .iter()
                .map(|p| Coordinate(p.0, p.1, p.2))
                .collect(),
        };
        for _ in 0..6 {
            dimension.evolve();
        }
        assert_eq!(dimension.active_cubes.len(), 112);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_17").part_1(), 301);
    }
}
