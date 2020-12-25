//! Day 20

use crate::docking_data::parse_integer;
use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{char, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TileId(usize);
type Tile<'a> = Vec<&'a str>;

fn find_corners(tiles: &[(TileId, Tile)]) -> Vec<TileId> {
    let mut borders = HashMap::<_, Vec<_>>::new();
    for (id, tile) in tiles {
        let top = tile.first().unwrap().to_string();
        let bottom = tile.last().unwrap().to_string();
        let left = tile.iter().map(|row| row.chars().next().unwrap()).join("");
        let right = tile.iter().map(|row| row.chars().last().unwrap()).join("");
        borders.entry(reversed(&top)).or_default().push(*id);
        borders.entry(reversed(&bottom)).or_default().push(*id);
        borders.entry(reversed(&left)).or_default().push(*id);
        borders.entry(reversed(&right)).or_default().push(*id);
        borders.entry(top).or_default().push(*id);
        borders.entry(bottom).or_default().push(*id);
        borders.entry(left).or_default().push(*id);
        borders.entry(right).or_default().push(*id);
    }
    let edges = borders
        .into_iter()
        .filter_map(|(_, tiles)| {
            if tiles.len() == 1 {
                Some(tiles[0])
            } else {
                None
            }
        })
        .collect_vec();
    edges
        .iter()
        .unique()
        .filter_map(|id| {
            if edges.iter().filter(|&i| i == id).count() == 4 {
                Some(*id)
            } else {
                None
            }
        })
        .collect_vec()
}
fn reversed(s: &str) -> String {
    s.chars().rev().collect()
}

fn parse_input(s: &str) -> IResult<&str, Vec<(TileId, Tile)>> {
    separated_list1(
        pair(line_ending, line_ending),
        separated_pair(
            delimited(tag("Tile "), map(parse_integer, |id| TileId(id)), char(':')),
            line_ending,
            parse_tile,
        ),
    )(s)
}
fn parse_tile(s: &str) -> IResult<&str, Tile> {
    separated_list1(line_ending, is_a("#."))(s)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let tiles = parse_input(self).expect("Failed to parse the input").1;
        find_corners(&tiles).iter().map(|id| id.0).product()
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
                    (
                        TileId(2311),
                        vec![
                            "..##.#..#.",
                            "##..#.....",
                            "#...##..#.",
                            "####.#...#",
                            "##.##.###.",
                            "##...#.###",
                            ".#.#.#..##",
                            "..#....#..",
                            "###...#.#.",
                            "..###..###",
                        ]
                    ),
                    (
                        TileId(1951),
                        vec![
                            "#.##...##.",
                            "#.####...#",
                            ".....#..##",
                            "#...######",
                            ".##.#....#",
                            ".###.#####",
                            "###.##.##.",
                            ".###....#.",
                            "..#.#..#.#",
                            "#...##.#..",
                        ]
                    ),
                    (
                        TileId(1171),
                        vec![
                            "####...##.",
                            "#..##.#..#",
                            "##.#..#.#.",
                            ".###.####.",
                            "..###.####",
                            ".##....##.",
                            ".#...####.",
                            "#.##.####.",
                            "####..#...",
                            ".....##...",
                        ]
                    ),
                    (
                        TileId(1427),
                        vec![
                            "###.##.#..",
                            ".#..#.##..",
                            ".#.##.#..#",
                            "#.#.#.##.#",
                            "....#...##",
                            "...##..##.",
                            "...#.#####",
                            ".#.####.#.",
                            "..#..###.#",
                            "..##.#..#.",
                        ]
                    ),
                    (
                        TileId(1489),
                        vec![
                            "##.#.#....",
                            "..##...#..",
                            ".##..##...",
                            "..#...#...",
                            "#####...#.",
                            "#..#.#.#.#",
                            "...#.#.#..",
                            "##.#...##.",
                            "..##.##.##",
                            "###.##.#..",
                        ]
                    ),
                    (
                        TileId(2473),
                        vec![
                            "#....####.",
                            "#..#.##...",
                            "#.##..#...",
                            "######.#.#",
                            ".#...#.#.#",
                            ".#########",
                            ".###.#..#.",
                            "########.#",
                            "##...##.#.",
                            "..###.#.#.",
                        ]
                    ),
                    (
                        TileId(2971),
                        vec![
                            "..#.#....#",
                            "#...###...",
                            "#.#.###...",
                            "##.##..#..",
                            ".#####..##",
                            ".#..####.#",
                            "#..#.#..#.",
                            "..####.###",
                            "..#.#.###.",
                            "...#.#.#.#",
                        ]
                    ),
                    (
                        TileId(2729),
                        vec![
                            "...#.#.#.#",
                            "####.#....",
                            "..#.#.....",
                            "....#..#.#",
                            ".##..##.#.",
                            ".#.####...",
                            "####.#.#..",
                            "##.####...",
                            "##..#.##..",
                            "#.##...##.",
                        ]
                    ),
                    (
                        TileId(3079),
                        vec![
                            "#.#.#####.",
                            ".#..######",
                            "..#.......",
                            "######....",
                            "####.#..#.",
                            ".#...#.##.",
                            "#.#####.##",
                            "..#.###...",
                            "..#.......",
                            "..#.###...",
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
            find_corners(&parse_input(EXAMPLE_INPUT).unwrap().1)
                .iter()
                .sorted(),
            &[TileId(1171), TileId(1951), TileId(2971), TileId(3079)],
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_20").part_1(), 107_399_567_124_539);
    }
}
