//! Day 23

use std::{collections::HashMap, hash::Hash};

use itertools::{iterate, Itertools};
use num_traits::{FromPrimitive, Num, NumAssignOps};

const SLICE_SIZE: usize = 3;
struct Game<T> {
    current: T,
    next_cup: HashMap<T, T>,
}
impl<T> Game<T>
where
    T: Copy + Eq + Ord + Hash + Num + NumAssignOps + FromPrimitive,
{
    pub fn new(mut cups: impl Iterator<Item = T>) -> Option<Self> {
        let current = cups.next()?;
        let mut prev = current;
        let mut next_cup = HashMap::new();
        while let Some(next) = cups.next() {
            next_cup.insert(prev, next);
            prev = next;
        }
        next_cup.insert(prev, current);
        Some(Self { current, next_cup })
    }
    pub fn do_move(&mut self) {
        // Draw three cups.
        let slice = self.next_slice();
        let move_start = *slice.first().unwrap();
        let move_end = *slice.last().unwrap();
        // Remove them.
        self.next_cup.insert(self.current, self.next_cup[&move_end]);
        // Select the destination cup.
        let dest_start = self.find_destination(self.current - T::one(), &slice);
        let dest_end = self.next_cup[&dest_start];
        // Place the cups.
        self.next_cup.insert(dest_start, move_start);
        self.next_cup.insert(move_end, dest_end);
        // Select a new current cup.
        self.current = self.next_cup[&self.current];
    }
    pub fn unroll<'a>(&'a self, from: &'a T) -> impl Iterator<Item = &'a T> {
        iterate(from, move |current| &self.next_cup[*current])
            .skip(1)
            .take_while(move |cup| cup != &from)
    }
    fn next_slice(&self) -> [T; SLICE_SIZE] {
        let mut cups = self.unroll(&self.current).copied();
        [
            cups.next().unwrap(),
            cups.next().unwrap(),
            cups.next().unwrap(),
        ]
    }
    fn find_destination(&self, from: T, slice: &[T; SLICE_SIZE]) -> T {
        let mut v = from;
        while v > T::zero() {
            if !slice.contains(&v) {
                return v;
            }
            v -= T::one();
        }
        self.find_destination(T::from_usize(self.next_cup.len()).unwrap(), slice)
    }
}

mod parsers {
    pub fn part_1<'a>(s: &'a str) -> impl Iterator<Item = u8> + 'a {
        s.bytes().map(|b| b - b'0')
    }
    pub fn part_2<'a>(s: &'a str) -> impl Iterator<Item = u32> + 'a {
        s.bytes()
            .map(|b| (b - b'0') as u32)
            .chain((s.len() as u32 + 1)..=1_000_000)
    }
}

trait Solution {
    fn part_1(&self) -> String;
    fn part_2(&self) -> u32;
}
impl Solution for &str {
    fn part_1(&self) -> String {
        let mut game = Game::new(parsers::part_1(self)).unwrap();
        for _ in 0..100 {
            game.do_move();
        }
        let ret = game.unroll(&1).join("");
        ret
    }
    fn part_2(&self) -> u32 {
        let mut game = Game::new(parsers::part_2(self)).unwrap();
        for _ in 0..10_000_000 {
            game.do_move();
        }
        game.unroll(&1).take(2).product()
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example_1() {
        let mut game = Game::new(parsers::part_1(&"389125467")).unwrap();
        assert_eq!(game.current, 3);
        game.do_move();
        assert_eq!(game.unroll(&3).join(""), "28915467");
        assert_eq!(game.current, 2);
        game.do_move();
        assert_eq!(game.unroll(&3).join(""), "25467891");
        assert_eq!(game.current, 5);
        for _ in 0..8 {
            game.do_move();
        }
        assert_eq!(game.unroll(&1).join(""), "92658374");
        for _ in 0..90 {
            game.do_move();
        }
        assert_eq!(game.unroll(&1).join(""), "67384529");
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_23").part_1(), "45286397");
    }

    #[test]
    fn example_2() {
        let mut game = Game::new(parsers::part_2(&"389125467")).unwrap();
        for _ in 0..10_000_000 {
            game.do_move()
        }
        assert_equal(game.unroll(&1).take(2), &[934001, 159792]);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_23").part_2(), 836_763_710);
    }
}
