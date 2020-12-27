//! Day 24

use std::collections::{HashMap, HashSet};

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        find_black_tiles(
            parsers::input(self)
                .expect("Failed to parse the input")
                .into_iter()
                .map(fold_axial_coordinates),
        )
        .len()
    }
    fn part_2(&self) -> usize {
        let mut floor = ArtExhibit {
            black_tiles: find_black_tiles(
                parsers::input(self)
                    .expect("Failed to parse the input")
                    .into_iter()
                    .map(fold_axial_coordinates),
            ),
        };
        for _ in 0..100 {
            floor.next_day();
        }
        floor.black_tiles.len()
    }
}

type Coord = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
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

trait CoordExt {
    fn step_towards(self, dir: Direction) -> Self;
}
impl CoordExt for Coord {
    fn step_towards(self, dir: Direction) -> Self {
        let offset = dir.into_axial_offset();
        (self.0 + offset.0, self.1 + offset.1)
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
    dirs.into_iter()
        .fold((0, 0), |tot, dir| tot.step_towards(dir))
}

struct ArtExhibit {
    pub black_tiles: HashSet<Coord>,
}
impl ArtExhibit {
    pub fn next_day(&mut self) {
        let mut to_white = Vec::new();
        let mut to_black = HashMap::<Coord, usize>::new();
        for tile in self.black_tiles.iter() {
            let neighbors = Self::neighbors(tile);
            let n_black = neighbors
                .iter()
                .filter(|&n| self.black_tiles.contains(n))
                .count();
            if n_black == 0 || n_black > 2 {
                to_white.push(*tile);
            }
            for tile in &neighbors {
                if !self.black_tiles.contains(&tile) {
                    *to_black.entry(*tile).or_default() += 1;
                }
            }
        }
        for tile in to_white {
            self.black_tiles.remove(&tile);
        }
        for (tile, n_black) in to_black {
            if n_black == 2 {
                self.black_tiles.insert(tile);
            }
        }
    }
    fn neighbors(tile: &Coord) -> [Coord; 6] {
        [
            tile.step_towards(Direction::East),
            tile.step_towards(Direction::SouthEast),
            tile.step_towards(Direction::SouthWest),
            tile.step_towards(Direction::West),
            tile.step_towards(Direction::NorthWest),
            tile.step_towards(Direction::NorthEast),
        ]
    }
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::line_ending,
        combinator::value,
        error::Error,
        multi::{many1, separated_list1},
        IResult,
    };

    use crate::parsers::finished_parser;

    use super::Direction;

    pub fn input(s: &str) -> Result<Vec<Vec<Direction>>, Error<&str>> {
        finished_parser(separated_list1(line_ending, many1(direction)))(s)
    }
    fn direction(s: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::East, tag("e")),
            value(Direction::SouthEast, tag("se")),
            value(Direction::SouthWest, tag("sw")),
            value(Direction::West, tag("w")),
            value(Direction::NorthWest, tag("nw")),
            value(Direction::NorthEast, tag("ne")),
        ))(s)
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
wseweeenwnesenwwwswnew"
                .part_1(),
            10
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_24").part_1(), 394);
    }

    #[test]
    fn example_2() {
        let mut floor = ArtExhibit {
            black_tiles: find_black_tiles(
                parsers::input(
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
wseweeenwnesenwwwswnew",
                )
                .unwrap()
                .into_iter()
                .map(fold_axial_coordinates),
            ),
        };
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 15);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 12);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 25);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 14);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 23);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 28);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 41);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 37);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 49);
        floor.next_day();
        assert_eq!(floor.black_tiles.len(), 37);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 132);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 259);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 406);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 566);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 788);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 1106);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 1373);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 1844);
        for _ in 0..10 {
            floor.next_day();
        }
        assert_eq!(floor.black_tiles.len(), 2208);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_24").part_2(), 4036);
    }
}
