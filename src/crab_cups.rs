//! Day 23

use std::collections::HashMap;

use itertools::{iterate, Itertools};

struct Game {
    current: u8,
    next_cup: HashMap<u8, u8>,
}
impl Game {
    pub fn new(mut cups: impl Iterator<Item = u8>) -> Option<Self> {
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
        let move_start = self.next_cup[&self.current];
        let move_end = self.nth(&move_start, 2);
        // Remove them.
        self.next_cup.insert(self.current, self.next_cup[&move_end]);
        // Select the destination cup.
        let dest_start = self.find_destination();
        let dest_end = self.next_cup[&dest_start];
        // Place the cups.
        self.next_cup.insert(dest_start, move_start);
        self.next_cup.insert(move_end, dest_end);
        // Select a new current cup.
        self.current = self.next_cup[&self.current];
    }
    pub fn unroll<'a>(&'a self, from: &'a u8) -> impl Iterator<Item = &'a u8> {
        iterate(from, move |current| &self.next_cup[*current])
            .skip(1)
            .take_while(move |cup| cup != &from)
    }
    fn nth(&self, from: &u8, n: usize) -> u8 {
        if n == 0 {
            *from
        } else {
            self.nth(&self.next_cup[from], n - 1)
        }
    }
    fn find_destination(&self) -> u8 {
        let valid = self.unroll(&self.current).copied().collect_vec();
        let mut v = self.current;
        let (min, max) = valid.iter().minmax().into_option().unwrap();
        while &v > min {
            v -= 1;
            if valid.contains(&v) {
                return v;
            }
        }
        *max
    }
}

trait Solution {
    fn part_1(&self) -> String;
}
impl Solution for &str {
    fn part_1(&self) -> String {
        let mut game = Game::new(self.bytes()).unwrap();
        for _ in 0..100 {
            game.do_move();
        }
        String::from_utf8(game.unroll(&b'1').copied().collect_vec())
            .expect("Failed to parse the input")
    }
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example_1() {
        let mut game = Game::new("389125467".bytes()).unwrap();
        assert_eq!(game.current, b'3');
        game.do_move();
        assert_equal(game.unroll(&b'3'), b"28915467");
        assert_eq!(game.current, b'2');
        game.do_move();
        assert_equal(game.unroll(&b'3'), b"25467891");
        assert_eq!(game.current, b'5');
        for _ in 0..8 {
            game.do_move();
        }
        assert_equal(game.unroll(&b'1'), b"92658374");
        for _ in 0..90 {
            game.do_move();
        }
        assert_equal(game.unroll(&b'1'), b"67384529");
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_23").part_1(), "45286397");
    }
}
