//! Day 24

use std::collections::HashSet;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

type Coord = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
impl Direction {
    pub fn into_axial_offset(self) -> Coord {
        match self {
            Self::East => (0, 1),
            Self::SouthEast => (1, 0),
            Self::SouthWest => (1, -1),
            Self::West => (0, -1),
            Self::NorthWest => (-1, 0),
            Self::NorthEast => (-1, 1),
        }
    }
}

fn find_black_tiles(flips: impl Iterator<Item = Coord>) -> HashSet<Coord> {
    let mut flipped = HashSet::new();
    for flip in flips {
        if flipped.contains(&flip) {
            flipped.remove(&flip);
        } else {
            flipped.insert(flip);
        }
    }
    flipped
}
fn fold_axial_coordinates(dirs: impl IntoIterator<Item = Direction>) -> Coord {
    dirs.into_iter().fold((0, 0), |tot, dir| {
        let offset = dir.into_axial_offset();
        (tot.0 + offset.0, tot.1 + offset.1)
    })
}

fn parse_input(s: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list1(line_ending, many1(parse_direction))(s)
}
fn parse_direction(s: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::East, tag("e")),
        value(Direction::SouthEast, tag("se")),
        value(Direction::SouthWest, tag("sw")),
        value(Direction::West, tag("w")),
        value(Direction::NorthWest, tag("nw")),
        value(Direction::NorthEast, tag("ne")),
    ))(s)
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        find_black_tiles(
            parse_input(self)
                .expect("Failed to parse the input")
                .1
                .into_iter()
                .map(fold_axial_coordinates),
        )
        .len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew
"
            .part_1(),
            10
        );
    }
    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_24").part_1(), 142);
    }
}
