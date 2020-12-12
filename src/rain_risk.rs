//! Day 12

use nom::{
    branch::alt,
    character::complete::digit1,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map_res, value},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

struct Ship {
    heading: Cardinal,
    east: i32,
    north: i32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Cardinal(Cardinal),
    Relative(Relative),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cardinal {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Relative {
    Forward,
    Right,
    Left,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            heading: Cardinal::East,
            east: 0,
            north: 0,
        }
    }
    pub fn do_move(self, direction: Direction, amount: i32) -> Self {
        match direction {
            Direction::Cardinal(direction) => self.move_cardinal(direction, amount),
            Direction::Relative(direction) => self.move_relative(direction, amount),
        }
    }
    pub fn move_cardinal(self, direction: Cardinal, amount: i32) -> Self {
        match direction {
            Cardinal::North => Self {
                north: self.north + amount,
                ..self
            },
            Cardinal::South => Self {
                north: self.north - amount,
                ..self
            },
            Cardinal::East => Self {
                east: self.east + amount,
                ..self
            },
            Cardinal::West => Self {
                east: self.east - amount,
                ..self
            },
        }
    }
    pub fn move_relative(self, direction: Relative, amount: i32) -> Self {
        match direction {
            Relative::Forward => {
                let direction = self.heading;
                self.move_cardinal(direction, amount)
            }
            Relative::Right => {
                assert_eq!(amount % 90, 0, "Only multiples of 90° are supported");
                let mut heading = self.heading;
                for _ in 0..amount / 90 {
                    heading = heading.rotate_right();
                }
                Self { heading, ..self }
            }
            Relative::Left => {
                assert_eq!(amount % 90, 0, "Only multiples of 90° are supported");
                let mut heading = self.heading;
                for _ in 0..amount / 90 {
                    heading = heading.rotate_left();
                }
                Self { heading, ..self }
            }
        }
    }
    pub fn get_manhattan_distance(&self) -> u32 {
        self.east.abs() as u32 + self.north.abs() as u32
    }
}
impl Cardinal {
    pub fn rotate_right(self) -> Self {
        match self {
            Cardinal::North => Cardinal::East,
            Cardinal::South => Cardinal::West,
            Cardinal::East => Cardinal::South,
            Cardinal::West => Cardinal::North,
        }
    }
    pub fn rotate_left(self) -> Self {
        match self {
            Cardinal::North => Cardinal::West,
            Cardinal::South => Cardinal::East,
            Cardinal::East => Cardinal::North,
            Cardinal::West => Cardinal::South,
        }
    }
}

fn parse_input(s: &str) -> IResult<&str, Vec<(Direction, i32)>> {
    all_consuming(separated_list0(
        line_ending,
        tuple((
            alt((
                value(Direction::Cardinal(Cardinal::North), char('N')),
                value(Direction::Cardinal(Cardinal::South), char('S')),
                value(Direction::Cardinal(Cardinal::East), char('E')),
                value(Direction::Cardinal(Cardinal::West), char('W')),
                value(Direction::Relative(Relative::Forward), char('F')),
                value(Direction::Relative(Relative::Right), char('R')),
                value(Direction::Relative(Relative::Left), char('L')),
            )),
            map_res(digit1, |s: &str| s.parse()),
        )),
    ))(s)
}

trait Solution {
    fn part_1(&self) -> u32;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        let mut ship = Ship::new();
        for (direction, amount) in parse_input(self).expect("Failed to parse the input").1 {
            ship = ship.do_move(direction, amount);
        }
        ship.get_manhattan_distance()
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
F10
N3
F7
R90
F11"
            ),
            Ok((
                "",
                vec![
                    (Direction::Relative(Relative::Forward), 10),
                    (Direction::Cardinal(Cardinal::North), 3),
                    (Direction::Relative(Relative::Forward), 7),
                    (Direction::Relative(Relative::Right), 90),
                    (Direction::Relative(Relative::Forward), 11),
                ]
            ))
        )
    }

    #[test]
    fn example_1() {
        let mut ship = Ship::new();
        ship = ship.move_relative(Relative::Forward, 10);
        assert_eq!(ship.east, 10);
        assert_eq!(ship.north, 0);
        ship = ship.move_cardinal(Cardinal::North, 3);
        assert_eq!(ship.east, 10);
        assert_eq!(ship.north, 3);
        ship = ship.move_relative(Relative::Forward, 7);
        assert_eq!(ship.east, 17);
        assert_eq!(ship.north, 3);
        ship = ship.move_relative(Relative::Right, 90);
        assert_eq!(ship.heading, Cardinal::South);
        ship = ship.move_relative(Relative::Forward, 11);
        assert_eq!(ship.east, 17);
        assert_eq!(ship.north, -8);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_12").part_1(), 1457);
    }
}
