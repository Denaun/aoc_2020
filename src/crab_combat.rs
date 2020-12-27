//! Day 22

use std::collections::{HashSet, VecDeque};

type Card = u64;
struct Combat {
    player_1: Deck,
    player_2: Deck,
}
struct RecursiveCombat {
    player_1: Deck,
    player_2: Deck,
    history_1: HashSet<Deck>,
    history_2: HashSet<Deck>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Deck(pub VecDeque<Card>);

trait Game {
    fn play_round(&mut self);
    fn has_winner(&self) -> bool;
    fn winner_score(&self) -> Card;
}
trait GameExt: Game {
    fn play_out(&mut self) {
        while !self.has_winner() {
            self.play_round()
        }
    }
}
impl<T: Game> GameExt for T {}

impl Combat {
    pub fn new(
        player_1: impl IntoIterator<Item = Card>,
        player_2: impl IntoIterator<Item = Card>,
    ) -> Self {
        Self {
            player_1: Deck(player_1.into_iter().collect()),
            player_2: Deck(player_2.into_iter().collect()),
        }
    }
}
impl Game for Combat {
    fn play_round(&mut self) {
        let card_1 = self.player_1.0.pop_front().unwrap();
        let card_2 = self.player_2.0.pop_front().unwrap();
        if card_1 > card_2 {
            self.player_1.0.push_back(card_1);
            self.player_1.0.push_back(card_2);
        } else {
            self.player_2.0.push_back(card_2);
            self.player_2.0.push_back(card_1);
        }
    }
    fn has_winner(&self) -> bool {
        self.player_1.0.is_empty() || self.player_2.0.is_empty()
    }
    fn winner_score(&self) -> Card {
        if self.player_1.0.is_empty() {
            self.player_2.score()
        } else {
            self.player_1.score()
        }
    }
}
impl RecursiveCombat {
    pub fn new(
        player_1: impl IntoIterator<Item = Card>,
        player_2: impl IntoIterator<Item = Card>,
    ) -> Self {
        Self {
            player_1: Deck(player_1.into_iter().collect()),
            player_2: Deck(player_2.into_iter().collect()),
            history_1: HashSet::new(),
            history_2: HashSet::new(),
        }
    }
    fn player_1_wins(&self) -> bool {
        self.has_already_been_played() || self.player_2.0.is_empty()
    }
    fn has_already_been_played(&self) -> bool {
        self.history_1.contains(&self.player_1) || self.history_2.contains(&self.player_2)
    }
}
impl Game for RecursiveCombat {
    fn play_round(&mut self) {
        self.history_1.insert(self.player_1.clone());
        self.history_2.insert(self.player_2.clone());
        let card_1 = self.player_1.0.pop_front().unwrap();
        let card_2 = self.player_2.0.pop_front().unwrap();
        let player_1_wins = if card_1 as usize <= self.player_1.0.len()
            && card_2 as usize <= self.player_2.0.len()
        {
            let mut sub_game = RecursiveCombat::new(
                self.player_1.0.iter().copied().take(card_1 as usize),
                self.player_2.0.iter().copied().take(card_2 as usize),
            );
            sub_game.play_out();
            sub_game.player_1_wins()
        } else {
            card_1 > card_2
        };
        if player_1_wins {
            self.player_1.0.push_back(card_1);
            self.player_1.0.push_back(card_2);
        } else {
            self.player_2.0.push_back(card_2);
            self.player_2.0.push_back(card_1);
        }
    }
    fn has_winner(&self) -> bool {
        self.has_already_been_played() || self.player_1.0.is_empty() || self.player_2.0.is_empty()
    }
    fn winner_score(&self) -> Card {
        if self.player_1.0.is_empty() {
            self.player_2.score()
        } else {
            self.player_1.score()
        }
    }
}
impl Deck {
    pub fn score(&self) -> Card {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(ix, card)| (ix as Card + 1) * card)
            .sum()
    }
}

mod parsers {
    use nom::{
        bytes::streaming::tag,
        character::complete::line_ending,
        error::Error,
        multi::separated_list1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    use crate::parsers::{double_line_ending, finished_parser, integer};

    use super::Card;

    pub fn input(s: &str) -> Result<(Vec<Card>, Vec<Card>), Error<&str>> {
        finished_parser(separated_pair(deck("1"), double_line_ending, deck("2")))(s)
    }
    fn deck(player: impl Into<String>) -> impl FnMut(&str) -> IResult<&str, Vec<Card>> {
        let player = player.into();
        move |s| {
            let (s, _) = tag("Player ")(s)?;
            let (s, _) = tag(player.as_str())(s)?;
            let (s, _) = terminated(tag(":"), line_ending)(s)?;
            separated_list1(line_ending, integer)(s)
        }
    }
}

trait Solution {
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        let (player_1, player_2) = parsers::input(self).expect("Failed to parse the input");
        let mut game = Combat::new(player_1, player_2);
        game.play_out();
        game.winner_score()
    }
    fn part_2(&self) -> u64 {
        let (player_1, player_2) = parsers::input(self).expect("Failed to parse the input");
        let mut game = RecursiveCombat::new(player_1, player_2);
        game.play_out();
        game.winner_score()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
            .part_1(),
            306
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_22").part_1(), 31_809);
    }

    #[test]
    fn example_2() {
        let (player_1, player_2) = parsers::input(
            "\
Player 1:
43
19

Player 2:
2
29
14",
        )
        .unwrap();
        let mut game = RecursiveCombat::new(player_1, player_2);
        for _ in 0..7 {
            game.play_round();
        }
        assert!(game.has_already_been_played());
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
            .part_2(),
            291
        )
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_22").part_2(), 32_835);
    }
}
