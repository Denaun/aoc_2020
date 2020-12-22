//! Day 22

use std::collections::VecDeque;

use nom::{
    bytes::streaming::tag,
    character::complete::line_ending,
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated},
    IResult,
};

use crate::docking_data::parse_integer;

type Card = u64;
struct Game {
    player_1: Deck,
    player_2: Deck,
}
struct Deck(pub VecDeque<Card>);

impl Game {
    pub fn new(
        player_1: impl IntoIterator<Item = Card>,
        player_2: impl IntoIterator<Item = Card>,
    ) -> Self {
        Self {
            player_1: Deck(player_1.into_iter().collect()),
            player_2: Deck(player_2.into_iter().collect()),
        }
    }
    pub fn play_round(&mut self) {
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
    pub fn has_winner(&self) -> bool {
        self.player_1.0.is_empty() || self.player_2.0.is_empty()
    }
    pub fn winner_score(&self) -> Card {
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

fn parse_input(s: &str) -> IResult<&str, (Vec<Card>, Vec<Card>)> {
    separated_pair(
        parse_deck("1"),
        pair(line_ending, line_ending),
        parse_deck("2"),
    )(s)
}
fn parse_deck(player: impl Into<String>) -> impl FnMut(&str) -> IResult<&str, Vec<Card>> {
    let player = player.into();
    move |s| {
        let (s, _) = tag("Player ")(s)?;
        let (s, _) = tag(player.as_str())(s)?;
        let (s, _) = terminated(tag(":"), line_ending)(s)?;
        separated_list1(line_ending, parse_integer)(s)
    }
}

trait Solution {
    fn part_1(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        let (player_1, player_2) = parse_input(self).expect("Failed to parse the input").1;
        let mut game = Game::new(player_1, player_2);
        while !game.has_winner() {
            game.play_round()
        }
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
}
