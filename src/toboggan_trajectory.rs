/// Day 3
use nom::{alt, character::complete::char, many0, map, named, separated_list0};

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

named!(parse_cell<&str, bool>, alt!(map!(char('.'), |_| false) | map!(char('#'), |_| true)));
named!(
    parse_input<&str, Vec<Vec<bool>>>,
    separated_list0!(char('\n'), many0!(parse_cell))
);

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        trajectory(
            (0, 0),
            &parse_input(self).expect("Failed to parse the input").1,
            (3, 1),
        )
        .filter(|v| *v)
        .count()
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
}
