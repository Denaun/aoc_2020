/// Day 1
use num_traits::Num;
use std::str::FromStr;

pub fn find_sum<T: Num + Copy>(values: &[T], sum: T) -> Option<(T, T)> {
    values[..values.len() - 1]
        .iter()
        .enumerate()
        .find_map(|(ix, &fst)| {
            let target = sum - fst;
            if let Some(snd) = values[ix + 1..].iter().copied().find(|&snd| snd == target) {
                Some((fst, snd))
            } else {
                None
            }
        })
}

pub fn parse_input<T: FromStr>(text: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    text.lines().map(str::parse).collect()
}

trait Solution {
    fn part_1(&self) -> u32;
}
impl Solution for str {
    fn part_1(&self) -> u32 {
        let (fst, snd) = find_sum(&parse_input(self).expect("Failed to parse the input"), 2020)
            .expect("Solution not found");
        fst * snd
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
1721
979
366
299
675
1456"
            ),
            Ok(vec![1721, 979, 366, 299, 675, 1456])
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            find_sum(&[1721, 979, 366, 299, 675, 1456], 2020),
            Some((1721, 299))
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_1").part_1(), 776064);
    }
}
