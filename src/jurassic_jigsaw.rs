//! Day 20

use crate::docking_data::parse_integer;
use bitvec::prelude::*;
use itertools::{iterate, Itertools};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tile {
    id: usize,
    data: Vec<BitVec>,
    sides: HashSet<BitVec>,
    sides_reversed: HashSet<BitVec>,
}
impl Tile {
    pub fn new(id: usize, data: Vec<BitVec>) -> Self {
        let sides = Orientation::iter()
            .map(|o| Self::side_facing(&data, o))
            .collect::<HashSet<_>>();
        let sides_reversed = sides
            .iter()
            .map(|side| side.iter().rev().copied().collect())
            .collect();
        Self {
            id,
            data,
            sides,
            sides_reversed,
        }
    }
    pub fn shared_side_count(&self, tiles: &[Tile]) -> usize {
        self.sides
            .iter()
            .map(|side| {
                tiles
                    .iter()
                    .filter(|tile| tile.id != self.id && tile.has_side(side))
                    .count()
            })
            .sum()
    }
    fn has_side(&self, side: &BitSlice) -> bool {
        self.sides.contains(side) || self.sides_reversed.contains(side)
    }

    fn side_facing(data: &[BitVec], orientation: Orientation) -> BitVec {
        match orientation {
            Orientation::Top => data.first().unwrap().iter().collect(),
            Orientation::Bottom => data.last().unwrap().iter().collect(),
            Orientation::Left => data.iter().map(|row| row.first().unwrap()).collect(),
            Orientation::Right => data.iter().map(|row| row.last().unwrap()).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}
impl Orientation {
    fn iter() -> impl Iterator<Item = Self> + Clone {
        [Self::Top, Self::Bottom, Self::Left, Self::Right]
            .iter()
            .copied()
    }
}

fn corners<'a>(tiles: &'a [Tile]) -> impl Iterator<Item = &'a Tile> {
    tiles
        .iter()
        .filter(move |tile| tile.shared_side_count(tiles) == 2)
}

fn parse_input(s: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(
        pair(line_ending, line_ending),
        map(
            separated_pair(
                delimited(tag("Tile "), parse_integer, char(':')),
                line_ending,
                parse_tile_data,
            ),
            |(id, data)| Tile::new(id, data),
        ),
    )(s)
}
fn parse_tile_data(s: &str) -> IResult<&str, Vec<BitVec>> {
    separated_list1(
        line_ending,
        map(
            many1(alt((value(true, char('#')), value(false, char('.'))))),
            |bits| bits.into_iter().collect(),
        ),
    )(s)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let tiles = parse_input(self).expect("Failed to parse the input").1;
        corners(&tiles).map(|tile| tile.id).product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    const EXAMPLE_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(EXAMPLE_INPUT),
            Ok((
                "",
                [
                    Tile::new(
                        2311,
                        vec![
                            bitvec![0, 0, 1, 1, 0, 1, 0, 0, 1, 0,],
                            bitvec![1, 1, 0, 0, 1, 0, 0, 0, 0, 0,],
                            bitvec![1, 0, 0, 0, 1, 1, 0, 0, 1, 0,],
                            bitvec![1, 1, 1, 1, 0, 1, 0, 0, 0, 1,],
                            bitvec![1, 1, 0, 1, 1, 0, 1, 1, 1, 0,],
                            bitvec![1, 1, 0, 0, 0, 1, 0, 1, 1, 1,],
                            bitvec![0, 1, 0, 1, 0, 1, 0, 0, 1, 1,],
                            bitvec![0, 0, 1, 0, 0, 0, 0, 1, 0, 0,],
                            bitvec![1, 1, 1, 0, 0, 0, 1, 0, 1, 0,],
                            bitvec![0, 0, 1, 1, 1, 0, 0, 1, 1, 1,],
                        ]
                    ),
                    Tile::new(
                        1951,
                        vec![
                            bitvec![1, 0, 1, 1, 0, 0, 0, 1, 1, 0,],
                            bitvec![1, 0, 1, 1, 1, 1, 0, 0, 0, 1,],
                            bitvec![0, 0, 0, 0, 0, 1, 0, 0, 1, 1,],
                            bitvec![1, 0, 0, 0, 1, 1, 1, 1, 1, 1,],
                            bitvec![0, 1, 1, 0, 1, 0, 0, 0, 0, 1,],
                            bitvec![0, 1, 1, 1, 0, 1, 1, 1, 1, 1,],
                            bitvec![1, 1, 1, 0, 1, 1, 0, 1, 1, 0,],
                            bitvec![0, 1, 1, 1, 0, 0, 0, 0, 1, 0,],
                            bitvec![0, 0, 1, 0, 1, 0, 0, 1, 0, 1,],
                            bitvec![1, 0, 0, 0, 1, 1, 0, 1, 0, 0,],
                        ]
                    ),
                    Tile::new(
                        1171,
                        vec![
                            bitvec![1, 1, 1, 1, 0, 0, 0, 1, 1, 0,],
                            bitvec![1, 0, 0, 1, 1, 0, 1, 0, 0, 1,],
                            bitvec![1, 1, 0, 1, 0, 0, 1, 0, 1, 0,],
                            bitvec![0, 1, 1, 1, 0, 1, 1, 1, 1, 0,],
                            bitvec![0, 0, 1, 1, 1, 0, 1, 1, 1, 1,],
                            bitvec![0, 1, 1, 0, 0, 0, 0, 1, 1, 0,],
                            bitvec![0, 1, 0, 0, 0, 1, 1, 1, 1, 0,],
                            bitvec![1, 0, 1, 1, 0, 1, 1, 1, 1, 0,],
                            bitvec![1, 1, 1, 1, 0, 0, 1, 0, 0, 0,],
                            bitvec![0, 0, 0, 0, 0, 1, 1, 0, 0, 0,],
                        ]
                    ),
                    Tile::new(
                        1427,
                        vec![
                            bitvec![1, 1, 1, 0, 1, 1, 0, 1, 0, 0,],
                            bitvec![0, 1, 0, 0, 1, 0, 1, 1, 0, 0,],
                            bitvec![0, 1, 0, 1, 1, 0, 1, 0, 0, 1,],
                            bitvec![1, 0, 1, 0, 1, 0, 1, 1, 0, 1,],
                            bitvec![0, 0, 0, 0, 1, 0, 0, 0, 1, 1,],
                            bitvec![0, 0, 0, 1, 1, 0, 0, 1, 1, 0,],
                            bitvec![0, 0, 0, 1, 0, 1, 1, 1, 1, 1,],
                            bitvec![0, 1, 0, 1, 1, 1, 1, 0, 1, 0,],
                            bitvec![0, 0, 1, 0, 0, 1, 1, 1, 0, 1,],
                            bitvec![0, 0, 1, 1, 0, 1, 0, 0, 1, 0,],
                        ]
                    ),
                    Tile::new(
                        1489,
                        vec![
                            bitvec![1, 1, 0, 1, 0, 1, 0, 0, 0, 0,],
                            bitvec![0, 0, 1, 1, 0, 0, 0, 1, 0, 0,],
                            bitvec![0, 1, 1, 0, 0, 1, 1, 0, 0, 0,],
                            bitvec![0, 0, 1, 0, 0, 0, 1, 0, 0, 0,],
                            bitvec![1, 1, 1, 1, 1, 0, 0, 0, 1, 0,],
                            bitvec![1, 0, 0, 1, 0, 1, 0, 1, 0, 1,],
                            bitvec![0, 0, 0, 1, 0, 1, 0, 1, 0, 0,],
                            bitvec![1, 1, 0, 1, 0, 0, 0, 1, 1, 0,],
                            bitvec![0, 0, 1, 1, 0, 1, 1, 0, 1, 1,],
                            bitvec![1, 1, 1, 0, 1, 1, 0, 1, 0, 0,],
                        ]
                    ),
                    Tile::new(
                        2473,
                        vec![
                            bitvec![1, 0, 0, 0, 0, 1, 1, 1, 1, 0,],
                            bitvec![1, 0, 0, 1, 0, 1, 1, 0, 0, 0,],
                            bitvec![1, 0, 1, 1, 0, 0, 1, 0, 0, 0,],
                            bitvec![1, 1, 1, 1, 1, 1, 0, 1, 0, 1,],
                            bitvec![0, 1, 0, 0, 0, 1, 0, 1, 0, 1,],
                            bitvec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1,],
                            bitvec![0, 1, 1, 1, 0, 1, 0, 0, 1, 0,],
                            bitvec![1, 1, 1, 1, 1, 1, 1, 1, 0, 1,],
                            bitvec![1, 1, 0, 0, 0, 1, 1, 0, 1, 0,],
                            bitvec![0, 0, 1, 1, 1, 0, 1, 0, 1, 0,],
                        ]
                    ),
                    Tile::new(
                        2971,
                        vec![
                            bitvec![0, 0, 1, 0, 1, 0, 0, 0, 0, 1,],
                            bitvec![1, 0, 0, 0, 1, 1, 1, 0, 0, 0,],
                            bitvec![1, 0, 1, 0, 1, 1, 1, 0, 0, 0,],
                            bitvec![1, 1, 0, 1, 1, 0, 0, 1, 0, 0,],
                            bitvec![0, 1, 1, 1, 1, 1, 0, 0, 1, 1,],
                            bitvec![0, 1, 0, 0, 1, 1, 1, 1, 0, 1,],
                            bitvec![1, 0, 0, 1, 0, 1, 0, 0, 1, 0,],
                            bitvec![0, 0, 1, 1, 1, 1, 0, 1, 1, 1,],
                            bitvec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0,],
                            bitvec![0, 0, 0, 1, 0, 1, 0, 1, 0, 1,],
                        ]
                    ),
                    Tile::new(
                        2729,
                        vec![
                            bitvec![0, 0, 0, 1, 0, 1, 0, 1, 0, 1,],
                            bitvec![1, 1, 1, 1, 0, 1, 0, 0, 0, 0,],
                            bitvec![0, 0, 1, 0, 1, 0, 0, 0, 0, 0,],
                            bitvec![0, 0, 0, 0, 1, 0, 0, 1, 0, 1,],
                            bitvec![0, 1, 1, 0, 0, 1, 1, 0, 1, 0,],
                            bitvec![0, 1, 0, 1, 1, 1, 1, 0, 0, 0,],
                            bitvec![1, 1, 1, 1, 0, 1, 0, 1, 0, 0,],
                            bitvec![1, 1, 0, 1, 1, 1, 1, 0, 0, 0,],
                            bitvec![1, 1, 0, 0, 1, 0, 1, 1, 0, 0,],
                            bitvec![1, 0, 1, 1, 0, 0, 0, 1, 1, 0,],
                        ]
                    ),
                    Tile::new(
                        3079,
                        vec![
                            bitvec![1, 0, 1, 0, 1, 1, 1, 1, 1, 0,],
                            bitvec![0, 1, 0, 0, 1, 1, 1, 1, 1, 1,],
                            bitvec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0,],
                            bitvec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0,],
                            bitvec![1, 1, 1, 1, 0, 1, 0, 0, 1, 0,],
                            bitvec![0, 1, 0, 0, 0, 1, 0, 1, 1, 0,],
                            bitvec![1, 0, 1, 1, 1, 1, 1, 0, 1, 1,],
                            bitvec![0, 0, 1, 0, 1, 1, 1, 0, 0, 0,],
                            bitvec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0,],
                            bitvec![0, 0, 1, 0, 1, 1, 1, 0, 0, 0,],
                        ]
                    )
                ]
                .iter()
                .cloned()
                .collect()
            ))
        );
    }

    #[test]
    fn example_1() {
        assert_equal(
            corners(&parse_input(EXAMPLE_INPUT).unwrap().1)
                .map(|tile| &tile.id)
                .sorted(),
            &[1171, 1951, 2971, 3079],
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_20").part_1(), 107_399_567_124_539);
    }
}
