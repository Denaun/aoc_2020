//! Day 3

use nom::{alt, character::complete::char, many0, map, named, separated_list0};
use std::ops::Mul;

fn trajectory<'a>(
    start: (usize, usize),
    map: &'a [impl AsRef<[bool]>],
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

fn count_trees(map: &[impl AsRef<[bool]>], slope: (usize, usize)) -> usize {
    trajectory((0, 0), &map, slope).filter(|v| *v).count()
}

named!(parse_cell<&str, bool>, alt!(map!(char('.'), |_| false) | map!(char('#'), |_| true)));
named!(
    parse_input<&str, Vec<Vec<bool>>>,
    separated_list0!(char('\n'), many0!(parse_cell))
);

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        count_trees(
            &parse_input(self).expect("Failed to parse the input").1,
            (3, 1),
        )
    }
    fn part_2(&self) -> usize {
        let map = &parse_input(self).expect("Failed to parse the input").1;
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|&slope| count_trees(&map, slope))
            .fold(1, Mul::mul)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_input() {
        assert_eq!(
            parse_input(".#\n##"),
            Ok(("", vec![vec![false, true], vec![true, true]]))
        )
    }

    #[test]
    fn example_1() {
        assert_eq!(
            trajectory(
                (0, 0),
                &parse_input(
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
                .unwrap()
                .1,
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
        let map = parse_input(
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
        .unwrap()
        .1;
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
