//! Day 15

use std::collections::HashMap;
use std::str::FromStr;

fn speak_numbers(initial: Vec<usize>) -> impl Iterator<Item = usize> {
    (0..).scan(
        (HashMap::new(), None),
        move |(last_turn_seen, previous), turn| {
            let current = if turn < initial.len() {
                initial[turn]
            } else {
                match &previous {
                    Some(previous) => last_turn_seen.get(previous).map(|v| turn - v).unwrap_or(0),
                    None => return None,
                }
            };
            if let Some(previous) = previous {
                last_turn_seen.insert(*previous, turn);
            };
            *previous = Some(current);
            Some(current)
        },
    )
}

fn parse_input<T: FromStr>(text: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    text.split(',').map(str::parse).collect()
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        speak_numbers(parse_input(self).expect("Failed to parse the input"))
            .nth(2019)
            .expect("Empty input")
    }
    fn part_2(&self) -> usize {
        speak_numbers(parse_input(self).expect("Failed to parse the input"))
            .nth(30_000_000 - 1)
            .expect("Empty input")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut numbers = speak_numbers(vec![0, 3, 6]);
        assert_eq!(numbers.next(), Some(0));
        assert_eq!(numbers.next(), Some(3));
        assert_eq!(numbers.next(), Some(6));
        assert_eq!(numbers.next(), Some(0));
        assert_eq!(numbers.next(), Some(3));
        assert_eq!(numbers.next(), Some(3));
        assert_eq!(numbers.next(), Some(1));
        assert_eq!(numbers.next(), Some(0));
        assert_eq!(numbers.next(), Some(4));
        assert_eq!(numbers.next(), Some(0));
    }

    #[test]
    fn example_2() {
        assert_eq!(speak_numbers(vec![1, 3, 2]).nth(2019), Some(1));
    }

    #[test]
    fn example_3() {
        assert_eq!(speak_numbers(vec![2, 1, 3]).nth(2019), Some(10));
    }

    #[test]
    fn example_4() {
        assert_eq!(speak_numbers(vec![1, 2, 3]).nth(2019), Some(27));
    }

    #[test]
    fn example_5() {
        assert_eq!(speak_numbers(vec![3, 2, 1]).nth(2019), Some(438));
    }

    #[test]
    fn example_6() {
        assert_eq!(speak_numbers(vec![3, 1, 2]).nth(2019), Some(1836));
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_15").part_1(), 421);
    }

    #[test]
    fn example_7() {
        assert_eq!(
            speak_numbers(vec![0, 3, 6]).nth(30_000_000 - 1),
            Some(175594)
        );
    }

    #[test]
    fn example_8() {
        assert_eq!(speak_numbers(vec![1, 3, 2]).nth(30_000_000 - 1), Some(2578));
    }

    #[test]
    fn example_9() {
        assert_eq!(
            speak_numbers(vec![2, 1, 3]).nth(30_000_000 - 1),
            Some(3544142)
        );
    }

    #[test]
    fn example_10() {
        assert_eq!(
            speak_numbers(vec![1, 2, 3]).nth(30_000_000 - 1),
            Some(261214)
        );
    }

    #[test]
    fn example_11() {
        assert_eq!(
            speak_numbers(vec![2, 3, 1]).nth(30_000_000 - 1),
            Some(6895259)
        );
    }

    #[test]
    fn example_12() {
        assert_eq!(speak_numbers(vec![3, 2, 1]).nth(30_000_000 - 1), Some(18));
    }

    #[test]
    fn example_13() {
        assert_eq!(speak_numbers(vec![3, 1, 2]).nth(30_000_000 - 1), Some(362));
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_15").part_2(), 436);
    }
}
