//! Day 3

use bitvec::prelude::*;

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        count_trees(
            &parsers::input(self).expect("Failed to parse the input"),
            (3, 1),
        )
    }
    fn part_2(&self) -> usize {
        let map = &parsers::input(self).expect("Failed to parse the input");
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&slope| count_trees(&map, slope))
            .product()
    }
}

fn trajectory<'a>(
    start: (usize, usize),
    map: &'a [impl AsRef<BitSlice>],
    slope: (usize, usize),
) -> impl Iterator<Item = bool> + 'a {
    let (from_left, from_top) = start;
    let (right_step, down_step) = slope;
    map.iter()
        .skip(from_top)
        .step_by(down_step)
        .scan(from_left, move |st, row| {
            let ix = *st;
            *st = (*st + right_step) % row.as_ref().len();
            Some(row.as_ref()[ix])
        })
}

fn count_trees(map: &[impl AsRef<BitSlice>], slope: (usize, usize)) -> usize {
    trajectory((0, 0), &map, slope).filter(|v| *v).count()
}

mod parsers {
    use bitvec::prelude::*;
    use nom::error::Error;

    use crate::parsers::{bw_image, finished_parser};

    pub fn input(s: &str) -> Result<Vec<BitVec>, Error<&str>> {
        finished_parser(bw_image)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        assert_eq!(
            parsers::input(".#\n##"),
            Ok(vec![bitvec![0, 1], bitvec![1, 1]])
        )
    }

    #[test]
    fn example_1() {
        assert_eq!(
            trajectory(
                (0, 0),
                &parsers::input(
                    "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
                )
                .unwrap(),
                (3, 1)
            )
            .enumerate()
            .filter_map(|(ix, v)| if v { Some(ix) } else { None })
            .collect::<Vec<_>>(),
            vec![2, 4, 5, 7, 8, 9, 10]
        )
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_3").part_1(), 230);
    }

    #[test]
    fn example_2() {
        let map = parsers::input(
            "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        )
        .unwrap();
        assert_eq!(count_trees(&map, (1, 1)), 2);
        assert_eq!(count_trees(&map, (3, 1)), 7);
        assert_eq!(count_trees(&map, (5, 1)), 3);
        assert_eq!(count_trees(&map, (7, 1)), 4);
        assert_eq!(count_trees(&map, (1, 2)), 2);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_3").part_2(), 9_533_698_720);
    }
}
