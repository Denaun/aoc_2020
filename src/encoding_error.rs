//! Day 9

use std::iter::FromIterator;

use itertools::Itertools;
use num_traits::{CheckedSub, NumAssignOps, Unsigned, Zero};

use crate::report_repair::find_sum;

trait Solution {
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        let input = parsers::input(self).expect("Failed to parse the input");
        find_first_non_valid(input[..25].iter().copied().collect(), &input[25..])
            .expect("Violation not found")
    }
    fn part_2(&self) -> u64 {
        let input: Vec<u64> = parsers::input(self).expect("Failed to parse the input");
        let sum = find_first_non_valid(input[..25].iter().copied().collect(), &input[25..])
            .expect("Violation not found");
        find_contiguous_sum(&input, sum)
            .expect("Weakness not found")
            .into_iter()
            .minmax()
            .into_option()
            .map(|(min, max)| min + max)
            .unwrap_or_default()
    }
}

fn is_valid<T: Zero + CheckedSub + Copy>(value: T, values: &impl AsRef<[T]>) -> bool {
    find_sum(values.as_ref(), value, 2).is_some()
}
fn find_first_non_valid<T: Zero + CheckedSub + Copy>(
    mut data: CircularBuffer<T>,
    values: &[T],
) -> Option<T> {
    for &v in values {
        if !is_valid(v, &data) {
            return Some(v);
        }
        data.push(v);
    }
    None
}
fn find_contiguous_sum<T: Unsigned + NumAssignOps + PartialOrd + Copy>(
    values: &[T],
    sum: T,
) -> Option<&[T]> {
    let mut current = T::zero();
    let mut start = 0;
    let mut end = 0;
    while current != sum {
        if end == values.len() {
            return None;
        }
        if current > sum {
            current -= values[start];
            start += 1;
            end = end.max(start);
        } else {
            current += values[end];
            end += 1;
        }
    }
    Some(&values[start..end])
}

#[derive(Debug, Clone)]
struct CircularBuffer<T> {
    buf: Vec<T>,
    ix: usize,
}
impl<T> CircularBuffer<T> {
    pub fn push(&mut self, v: T) {
        self.buf[self.ix] = v;
        self.ix = (self.ix + 1) % self.buf.len();
    }
}
impl<T> FromIterator<T> for CircularBuffer<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        CircularBuffer {
            buf: Vec::from_iter(iter),
            ix: 0,
        }
    }
}
impl<T> AsRef<[T]> for CircularBuffer<T> {
    fn as_ref(&self) -> &[T] {
        self.buf.as_ref()
    }
}

mod parsers {
    pub use crate::parsers::number_list as input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
                "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
            ),
            Ok(vec![
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ])
        );
    }

    #[test]
    fn example_1() {
        let mut data: CircularBuffer<_> =
            [20].iter().copied().chain(1..=19).chain(21..=25).collect();
        assert!(is_valid(26, &data));
        assert!(is_valid(49, &data));
        assert!(!is_valid(100, &data));
        assert!(!is_valid(50, &data));
        data.push(45);
        assert!(is_valid(26, &data));
        assert!(!is_valid(65, &data));
        assert!(is_valid(64, &data));
        assert!(is_valid(66, &data));
    }

    #[test]
    fn example_2() {
        let input = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(
            find_first_non_valid(input[..5].iter().copied().collect(), &input[5..]),
            Some(127)
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_9").part_1(), 3_199_139_634);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            find_contiguous_sum::<u32>(
                &[
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576,
                ],
                127
            ),
            Some(&[15, 25, 47, 40][..])
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_9").part_2(), 438_559_930);
    }
}
