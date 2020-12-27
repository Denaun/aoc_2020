//! Day 11

use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Layout {
    pub storage: Vec<Vec<Option<bool>>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LayoutPos {
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
    pub fn above_left(self, layout: &Layout) -> Option<LayoutPos> {
        self.above(layout).and_then(|pos| pos.left(layout))
    }
    pub fn above_right(self, layout: &Layout) -> Option<LayoutPos> {
        self.above(layout).and_then(|pos| pos.right(layout))
    }
    pub fn below_left(self, layout: &Layout) -> Option<LayoutPos> {
        self.below(layout).and_then(|pos| pos.left(layout))
    }
    pub fn below_right(self, layout: &Layout) -> Option<LayoutPos> {
        self.below(layout).and_then(|pos| pos.right(layout))
    }
}

impl Layout {
    pub fn simulate_shortsighted(&mut self) -> bool {
        let indices_to_mutate = self.determine_mutation(|pos| self.n_neighbors(pos), 4);
        self.apply_mutation(indices_to_mutate)
    }
    pub fn simulate_farsighted(&mut self) -> bool {
        let indices_to_mutate = self.determine_mutation(|pos| self.n_visible(pos), 5);
        self.apply_mutation(indices_to_mutate)
    }
    fn determine_mutation(
        &self,
        n_neighbors: impl Fn(LayoutPos) -> usize,
        threshold: usize,
    ) -> Vec<LayoutPos> {
        let mut indices_to_mutate: Vec<LayoutPos> = vec![];
        for row in 0..self.storage.len() {
            for col in 0..self.storage[row].len() {
                let pos = LayoutPos { row, col };
                let n_occupied_around = n_neighbors(pos);
                match self[pos] {
                    Some(occupied) => {
                        if occupied {
                            if n_occupied_around >= threshold {
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
        indices_to_mutate
    }
    fn apply_mutation(&mut self, indices_to_mutate: Vec<LayoutPos>) -> bool {
        let is_stable = indices_to_mutate.is_empty();
        for pos_mut in indices_to_mutate {
            self[pos_mut] = Some(!self[pos_mut].unwrap());
        }
        is_stable
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

    fn try_find(
        &self,
        mut pos: LayoutPos,
        get_next: impl Fn(LayoutPos) -> Option<LayoutPos>,
    ) -> Option<LayoutPos> {
        loop {
            if let Some(next) = get_next(pos) {
                if self[next].is_some() {
                    return Some(next);
                } else {
                    pos = next
                }
            } else {
                return None;
            }
        }
    }
    pub fn visible(&self, pos: LayoutPos) -> [Option<LayoutPos>; 8] {
        let above = self.try_find(pos, |pos| pos.above(self));
        let below = self.try_find(pos, |pos| pos.below(self));
        let left = self.try_find(pos, |pos| pos.left(self));
        let right = self.try_find(pos, |pos| pos.right(self));
        let above_left = self.try_find(pos, |pos| pos.above_left(self));
        let above_right = self.try_find(pos, |pos| pos.above_right(self));
        let below_left = self.try_find(pos, |pos| pos.below_left(self));
        let below_right = self.try_find(pos, |pos| pos.below_right(self));
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
    pub fn n_visible(&self, pos: LayoutPos) -> usize {
        self.visible(pos)
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

mod parsers {
    use nom::{
        branch::alt,
        character::complete::{char, line_ending},
        combinator::value,
        error::Error,
        multi::{many0, separated_list0},
        IResult,
    };

    use crate::parsers::finished_parser;

    use super::Layout;

    pub fn input(s: &str) -> Result<Layout, Error<&str>> {
        finished_parser(storage)(s).map(|storage| Layout { storage })
    }
    fn storage(s: &str) -> IResult<&str, Vec<Vec<Option<bool>>>> {
        separated_list0(
            line_ending,
            many0(alt((
                value(None, char('.')),
                value(Some(false), char('L')),
                value(Some(true), char('#')),
            ))),
        )(s)
    }
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        let mut layout = parsers::input(self).expect("Failed to parse the input");
        while !layout.simulate_shortsighted() {}
        layout.occupied()
    }
    fn part_2(&self) -> usize {
        let mut layout = parsers::input(self).expect("Failed to parse the input");
        while !layout.simulate_farsighted() {}
        layout.occupied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_input() {
        assert_eq!(
            parsers::input("L.L\nLL#"),
            Ok(Layout {
                storage: vec![
                    vec![Some(false), None, Some(false)],
                    vec![Some(false), Some(false), Some(true)],
                ]
            })
        );
    }

    #[test]
    fn example_1() {
        let mut layout = parsers::input(
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
        .unwrap();
        assert!(!layout.simulate_shortsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(!layout.simulate_shortsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(!layout.simulate_shortsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(!layout.simulate_shortsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(!layout.simulate_shortsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(layout.simulate_shortsighted());
        assert_eq!(layout.occupied(), 37);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_11").part_1(), 2489);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            parsers::input(
                "\
.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#....."
            )
            .unwrap()
            .n_visible(LayoutPos { row: 4, col: 3 }),
            8
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            parsers::input(
                "\
.............
.L.L.#.#.#.#.
............."
            )
            .unwrap()
            .visible(LayoutPos { row: 1, col: 1 })
            .iter()
            .flatten()
            .collect::<Vec<_>>(),
            vec![&LayoutPos { row: 1, col: 3 }]
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            parsers::input(
                "\
.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##."
            )
            .unwrap()
            .n_visible(LayoutPos { row: 3, col: 3 }),
            0
        );
    }

    #[test]
    fn example_5() {
        let mut layout = parsers::input(
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
        .unwrap();
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
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
        );
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
                "\
#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#"
            )
            .unwrap()
        );
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
                "\
#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#"
            )
            .unwrap()
        );
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
                "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#"
            )
            .unwrap()
        );
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
                "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
            )
            .unwrap()
        );
        assert!(!layout.simulate_farsighted());
        assert_eq!(
            layout,
            parsers::input(
                "\
#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#"
            )
            .unwrap()
        );
        assert!(layout.simulate_farsighted());
        assert_eq!(layout.occupied(), 26);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_11").part_2(), 2180);
    }
}
