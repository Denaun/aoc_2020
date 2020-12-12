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

struct HeadedShip {
    heading: Cardinal,
    position: Position,
}
struct WaypontedShip {
    waypoint: Position,
    position: Position,
}
struct Position {
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

trait Ship {
    fn move_cardinal(self, direction: Cardinal, amount: i32) -> Self;
    fn move_relative(self, direction: Relative, amount: i32) -> Self;
}
trait ShipExt: Ship + Sized {
    fn do_move(self, direction: Direction, amount: i32) -> Self {
        match direction {
            Direction::Cardinal(direction) => self.move_cardinal(direction, amount),
            Direction::Relative(direction) => self.move_relative(direction, amount),
        }
    }
}
impl<T: Ship> ShipExt for T {}

impl HeadedShip {
    pub fn new() -> Self {
        Self {
            heading: Cardinal::East,
            position: Position::default(),
        }
    }
}
impl Ship for HeadedShip {
    fn move_cardinal(self, direction: Cardinal, amount: i32) -> Self {
        Self {
            position: self.position.move_cardinal(direction, amount),
            ..self
        }
    }
    fn move_relative(self, direction: Relative, amount: i32) -> Self {
        match direction {
            Relative::Forward => {
                let direction = self.heading;
                self.move_cardinal(direction, amount)
            }
            Relative::Right => Self {
                heading: self.heading.rotate_degrees(true, amount),
                ..self
            },
            Relative::Left => Self {
                heading: self.heading.rotate_degrees(false, amount),
                ..self
            },
        }
    }
}

impl WaypontedShip {
    pub fn new() -> Self {
        Self {
            waypoint: Position { east: 10, north: 1 },
            position: Position::default(),
        }
    }
}
impl Ship for WaypontedShip {
    fn move_cardinal(self, direction: Cardinal, amount: i32) -> Self {
        Self {
            waypoint: self.waypoint.move_cardinal(direction, amount),
            ..self
        }
    }
    fn move_relative(self, direction: Relative, amount: i32) -> Self {
        match direction {
            Relative::Forward => {
                let mut position = self.position;
                for _ in 0..amount {
                    position = position
                        .move_cardinal(Cardinal::East, self.waypoint.east)
                        .move_cardinal(Cardinal::North, self.waypoint.north);
                }
                Self { position, ..self }
            }
            Relative::Right => Self {
                waypoint: self.waypoint.rotate_degrees(true, amount),
                ..self
            },
            Relative::Left => Self {
                waypoint: self.waypoint.rotate_degrees(false, amount),
                ..self
            },
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { east: 0, north: 0 }
    }
}
impl Position {
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
    pub fn get_manhattan_distance(&self) -> u32 {
        self.east.abs() as u32 + self.north.abs() as u32
    }
}

trait Rotatable {
    fn rotate_right(self) -> Self;
    fn rotate_left(self) -> Self;
}
impl Rotatable for Cardinal {
    fn rotate_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
    fn rotate_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}
impl Rotatable for Position {
    fn rotate_right(self) -> Self {
        Self {
            east: self.north,
            north: -self.east,
        }
    }
    fn rotate_left(self) -> Self {
        Self {
            east: -self.north,
            north: self.east,
        }
    }
}
trait RotatableExt: Rotatable + Sized {
    fn rotate_degrees(mut self, right: bool, degrees: i32) -> Self {
        assert_eq!(degrees % 90, 0, "Only multiples of 90° are supported");
        for _ in 0..degrees / 90 {
            self = if right {
                self.rotate_right()
            } else {
                self.rotate_left()
            };
        }
        self
    }
}
impl<T: Rotatable> RotatableExt for T {}

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
    fn part_2(&self) -> u32;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .into_iter()
            .fold(HeadedShip::new(), |ship, (direction, amount)| {
                ship.do_move(direction, amount)
            })
            .position
            .get_manhattan_distance()
    }
    fn part_2(&self) -> u32 {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .into_iter()
            .fold(WaypontedShip::new(), |ship, (direction, amount)| {
                ship.do_move(direction, amount)
            })
            .position
            .get_manhattan_distance()
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
        let mut ship = HeadedShip::new();
        ship = ship.move_relative(Relative::Forward, 10);
        assert_eq!(ship.position.east, 10);
        assert_eq!(ship.position.north, 0);
        ship = ship.move_cardinal(Cardinal::North, 3);
        assert_eq!(ship.position.east, 10);
        assert_eq!(ship.position.north, 3);
        ship = ship.move_relative(Relative::Forward, 7);
        assert_eq!(ship.position.east, 17);
        assert_eq!(ship.position.north, 3);
        ship = ship.move_relative(Relative::Right, 90);
        assert_eq!(ship.heading, Cardinal::South);
        ship = ship.move_relative(Relative::Forward, 11);
        assert_eq!(ship.position.east, 17);
        assert_eq!(ship.position.north, -8);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_12").part_1(), 1457);
    }

    #[test]
    fn example_2() {
        let mut ship = WaypontedShip::new();
        ship = ship.move_relative(Relative::Forward, 10);
        assert_eq!(ship.position.east, 100);
        assert_eq!(ship.position.north, 10);
        ship = ship.move_cardinal(Cardinal::North, 3);
        assert_eq!(ship.waypoint.east, 10);
        assert_eq!(ship.waypoint.north, 4);
        ship = ship.move_relative(Relative::Forward, 7);
        assert_eq!(ship.position.east, 170);
        assert_eq!(ship.position.north, 38);
        ship = ship.move_relative(Relative::Right, 90);
        assert_eq!(ship.waypoint.east, 4);
        assert_eq!(ship.waypoint.north, -10);
        ship = ship.move_relative(Relative::Forward, 11);
        assert_eq!(ship.position.east, 214);
        assert_eq!(ship.position.north, -72);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_12").part_2(), 106_860);
    }
}
