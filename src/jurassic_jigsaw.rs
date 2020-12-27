//! Day 20

use std::collections::HashSet;

use bitvec::prelude::*;
use itertools::{iterate, Itertools};

const SEA_MONSTER: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    pub id: usize,
    pub data: Vec<BitVec>,
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
    pub fn from_image(id: usize, tiles: Vec<Vec<Tile>>) -> Self {
        let rows_per_tile = tiles
            .first()
            .and_then(|row| row.first().map(|tile| tile.data.len()))
            .unwrap_or_default();
        Self::new(
            id,
            tiles
                .into_iter()
                .flat_map(|mut row| {
                    assert!(row.iter().all(|tile| tile.data.len() == rows_per_tile));
                    (1..rows_per_tile - 1).map(move |x| {
                        row.iter_mut()
                            .flat_map(|tile| {
                                tile.data[x].pop();
                                tile.data[x].iter().skip(1)
                            })
                            .collect()
                    })
                })
                .collect(),
        )
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

    pub fn is_side_shared(&self, orientation: Orientation, tiles: &[Tile]) -> bool {
        let side = Self::side_facing(&self.data, orientation);
        tiles
            .iter()
            .any(|tile| tile.id != self.id && tile.has_side(&side))
    }
    pub fn find_and_orient_neighbor(&self, my_side: Orientation, tiles: &[Tile]) -> Option<Tile> {
        let their_side = my_side.opposite();
        let my_side = Self::side_facing(&self.data, my_side);
        tiles
            .iter()
            .find(|tile| tile.id != self.id && tile.has_side(&my_side))
            .and_then(|tile| tile.clone().orient_to_side(&my_side, their_side))
    }
    fn orient_to_side(self, side: &BitSlice, orientation: Orientation) -> Option<Self> {
        self.orientations()
            .find(move |this| Self::side_facing(&this.data, orientation) == side)
    }
    pub fn orientations(self) -> impl Iterator<Item = Self> {
        iterate(self, |this| this.clone().flip_h())
            .take(2)
            .flat_map(|this| iterate(this, |this| this.clone().rotate_clockwise()).take(4))
    }
    fn flip_h(self) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .map(|mut row| {
                    row.reverse();
                    row
                })
                .collect(),
            ..self
        }
    }
    fn rotate_clockwise(self) -> Self {
        Self {
            data: (0..self.data.len())
                .map(|x| {
                    (0..self.data[x].len())
                        .rev()
                        .map(|y| self.data[y][x])
                        .collect()
                })
                .collect(),
            ..self
        }
    }

    pub fn mask_all(&mut self, mask: &[(usize, usize)]) -> bool {
        let max_x = mask.iter().map(|(x, _)| x).max();
        let max_y = mask.iter().map(|(_, y)| y).max();
        match (max_x, max_y) {
            (Some(max_x), Some(max_y)) => {
                let mut found = false;
                for x in 0..self.data.len() - max_x {
                    for y in 0..self.data.len() - max_y {
                        let actual_coords = mask.iter().map(|(x0, y0)| (x0 + x, y0 + y));
                        if actual_coords.clone().all(|(x, y)| self.data[x][y]) {
                            found = true;
                            for (x, y) in actual_coords {
                                self.data[x].set(y, false);
                            }
                        }
                    }
                }
                found
            }
            _ => false,
        }
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
pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
}
impl Orientation {
    pub fn iter() -> impl Iterator<Item = Self> + Clone {
        [Self::Top, Self::Bottom, Self::Left, Self::Right]
            .iter()
            .copied()
    }
    pub fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

// struct Orientations<'a> {
//     tile: Tile,
//     dummy: ,
// }

fn corners<'a>(tiles: &'a [Tile]) -> impl Iterator<Item = &'a Tile> {
    tiles
        .iter()
        .filter(move |tile| tile.shared_side_count(tiles) == 2)
}
fn build_image(tiles: &[Tile]) -> Option<Vec<Vec<Tile>>> {
    let top_left = corners(tiles)
        .next()?
        .clone()
        .orientations()
        .find(|tile| {
            !tile.is_side_shared(Orientation::Top, tiles)
                && !tile.is_side_shared(Orientation::Left, tiles)
        })
        .unwrap();

    let mut image = build_line(tiles, top_left, Orientation::Bottom)
        .map(|tile| vec![tile])
        .collect_vec();
    for row in &mut image {
        *row = build_line(tiles, row.pop().unwrap(), Orientation::Right).collect_vec();
    }
    Some(image)
}
fn build_line<'a>(
    tiles: &'a [Tile],
    start: Tile,
    side: Orientation,
) -> impl Iterator<Item = Tile> + 'a {
    iterate(Some(start), move |tile| {
        tile.as_ref()
            .and_then(|tile| tile.find_and_orient_neighbor(side, tiles))
    })
    .take_while(Option::is_some)
    .flatten()
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending},
        combinator::map,
        error::Error,
        multi::separated_list1,
        sequence::{delimited, separated_pair},
    };

    use crate::parsers::{bw_image, double_line_ending, finished_parser, integer};

    use super::Tile;

    pub fn input(s: &str) -> Result<Vec<Tile>, Error<&str>> {
        finished_parser(separated_list1(
            double_line_ending,
            map(
                separated_pair(
                    delimited(tag("Tile "), integer, char(':')),
                    line_ending,
                    bw_image,
                ),
                |(id, data)| Tile::new(id, data),
            ),
        ))(s)
    }
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let tiles = parsers::input(self).expect("Failed to parse the input");
        corners(&tiles).map(|tile| tile.id).product()
    }
    fn part_2(&self) -> usize {
        let tiles = parsers::input(self).expect("Failed to parse the input");
        let image = Tile::from_image(
            0,
            build_image(&tiles).expect("Failed to build the full image"),
        );
        image
            .orientations()
            .find_map(|mut tile| {
                if tile.mask_all(&SEA_MONSTER) {
                    Some(tile)
                } else {
                    None
                }
            })
            .expect("Failed to find any sea monster")
            .data
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|&b| b)
            .count()
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
            parsers::input(EXAMPLE_INPUT),
            Ok(vec![
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
            ])
        );
    }

    #[test]
    fn example_1() {
        assert_equal(
            corners(&parsers::input(EXAMPLE_INPUT).unwrap())
                .map(|tile| &tile.id)
                .sorted(),
            &[1171, 1951, 2971, 3079],
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_20").part_1(), 107_399_567_124_539);
    }

    #[test]
    fn example_2() {
        let image = Tile::from_image(
            0,
            build_image(&parsers::input(EXAMPLE_INPUT).unwrap()).unwrap(),
        )
        .orient_to_side(
            &bits![0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1,],
            Orientation::Top,
        )
        .unwrap();
        assert_eq!(
            image.data,
            vec![
                bitvec![0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1,],
                bitvec![1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,],
                bitvec![1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0,],
                bitvec![1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1,],
                bitvec![1, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 1,],
                bitvec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1,],
                bitvec![0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0,],
                bitvec![0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,],
                bitvec![1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0,],
                bitvec![1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 0,],
                bitvec![1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1,],
                bitvec![1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1,],
                bitvec![1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0,],
                bitvec![0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 1,],
                bitvec![0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 0,],
                bitvec![1, 1, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0,],
                bitvec![0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0, 0,],
                bitvec![0, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1,],
                bitvec![0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1,],
                bitvec![1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0,],
                bitvec![1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1,],
                bitvec![1, 0, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1,],
                bitvec![0, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0,],
                bitvec![0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1,],
            ]
        );
        assert_eq!(
            image
                .orientations()
                .find_map(|mut tile| if tile.mask_all(&SEA_MONSTER) {
                    Some(tile)
                } else {
                    None
                })
                .unwrap()
                .data
                .into_iter()
                .flat_map(|row| row.into_iter())
                .filter(|&b| b)
                .count(),
            273
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_20").part_2(), 1555);
    }
}
