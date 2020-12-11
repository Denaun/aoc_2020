//! Day 11

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    combinator::{all_consuming, value},
    multi::{many0, separated_list0},
    IResult,
};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Layout {
    pub storage: Vec<Vec<Option<bool>>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct LayoutPos {
    row: usize,
    col: usize,
}
impl LayoutPos {
    pub fn above(self, _layout: &Layout) -> Option<LayoutPos> {
        if self.row == 0 {
            None
        } else {
            Some(LayoutPos {
                row: self.row - 1,
                col: self.col,
            })
        }
    }
    pub fn below(self, layout: &Layout) -> Option<LayoutPos> {
        if self.row >= layout.storage.len() - 1 {
            None
        } else {
            Some(LayoutPos {
                row: self.row + 1,
                col: self.col,
            })
        }
    }
    pub fn left(self, _layout: &Layout) -> Option<LayoutPos> {
        if self.col == 0 {
            None
        } else {
            Some(LayoutPos {
                row: self.row,
                col: self.col - 1,
            })
        }
    }
    pub fn right(self, layout: &Layout) -> Option<LayoutPos> {
        if self.col >= layout.storage[self.row].len() - 1 {
            None
        } else {
            Some(LayoutPos {
                row: self.row,
                col: self.col + 1,
            })
        }
    }
}

impl Layout {
    pub fn simulate_round(&mut self) -> bool {
        let mut indices_to_mutate: Vec<LayoutPos> = vec![];
        for row in 0..self.storage.len() {
            for col in 0..self.storage[row].len() {
                let pos = LayoutPos { row, col };
                let n_occupied_around = self.n_neighbors(pos);
                match self[pos] {
                    Some(occupied) => {
                        if occupied {
                            if n_occupied_around >= 4 {
                                indices_to_mutate.push(pos);
                            }
                        } else {
                            if n_occupied_around == 0 {
                                indices_to_mutate.push(pos);
                            }
                        }
                    }
                    _ => continue,
                }
            }
        }
        let is_stable = indices_to_mutate.is_empty();
        for pos_mut in indices_to_mutate {
            self[pos_mut] = Some(!self[pos_mut].unwrap());
        }
        is_stable
    }
    pub fn simulate_until_stable(&mut self) {
        while !self.simulate_round() {}
    }
    pub fn occupied(&self) -> usize {
        self.storage
            .iter()
            .flat_map(|row| row.iter())
            .filter(|seat| matches!(seat, Some(true)))
            .count()
    }

    pub fn neighbors(&self, pos: LayoutPos) -> [Option<LayoutPos>; 8] {
        let above = pos.above(self);
        let below = pos.below(self);
        let left = pos.left(self);
        let right = pos.right(self);
        let above_left = above.and_then(|pos| pos.left(self));
        let above_right = above.and_then(|pos| pos.right(self));
        let below_left = below.and_then(|pos| pos.left(self));
        let below_right = below.and_then(|pos| pos.right(self));
        [
            above,
            below,
            left,
            right,
            above_left,
            above_right,
            below_left,
            below_right,
        ]
    }

    pub fn n_neighbors(&self, pos: LayoutPos) -> usize {
        self.neighbors(pos)
            .iter()
            .flatten()
            .filter(|&&p| matches!(self[p], Some(true)))
            .count()
    }
}

impl Index<LayoutPos> for Layout {
    type Output = Option<bool>;
    fn index(&self, pos: LayoutPos) -> &Self::Output {
        self.storage.index(pos.row).index(pos.col)
    }
}

impl IndexMut<LayoutPos> for Layout {
    fn index_mut(&mut self, pos: LayoutPos) -> &mut Self::Output {
        self.storage.index_mut(pos.row).index_mut(pos.col)
    }
}

fn parse_input(s: &str) -> IResult<&str, Layout> {
    let (s, storage) = all_consuming(separated_list0(
        line_ending,
        many0(alt((
            value(None, char('.')),
            value(Some(false), char('L')),
            value(Some(true), char('#')),
        ))),
    ))(s)?;
    Ok((s, Layout { storage }))
}

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let mut layout = parse_input(self).expect("Failed to parse the input").1;
        layout.simulate_until_stable();
        layout.occupied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_input() {
        assert_eq!(
            parse_input("L.L\nLL#"),
            Ok((
                "",
                Layout {
                    storage: vec![
                        vec![Some(false), None, Some(false)],
                        vec![Some(false), Some(false), Some(true)],
                    ]
                }
            ))
        );
    }

    #[test]
    fn example_1() {
        let mut layout = parse_input(
            "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
        .unwrap()
        .1;
        assert!(!layout.simulate_round());
        assert_eq!(
            layout,
            parse_input(
                "\
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"
            )
            .unwrap()
            .1
        );
        assert!(!layout.simulate_round());
        assert_eq!(
            layout,
            parse_input(
                "\
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##"
            )
            .unwrap()
            .1
        );
        assert!(!layout.simulate_round());
        assert_eq!(
            layout,
            parse_input(
                "\
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##"
            )
            .unwrap()
            .1
        );
        assert!(!layout.simulate_round());
        assert_eq!(
            layout,
            parse_input(
                "\
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##"
            )
            .unwrap()
            .1
        );
        assert!(!layout.simulate_round());
        assert_eq!(
            layout,
            parse_input(
                "\
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##"
            )
            .unwrap()
            .1
        );
        assert!(layout.simulate_round());
        assert_eq!(layout.occupied(), 37);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_11").part_1(), 2489);
    }
}
