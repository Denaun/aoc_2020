//! Day 5

use nom::{
    character::complete::{line_ending, one_of},
    multi::separated_list0,
    IResult,
};

trait Pass {
    fn id(&self) -> u16;
}
impl Pass for (u8, u8) {
    fn id(&self) -> u16 {
        self.0 as u16 * 8 + self.1 as u16
    }
}

fn parse_row(mut s: &str) -> IResult<&str, u8> {
    let mut row = 0;
    for i in (0..7).rev() {
        let (s1, b) = one_of("FB")(s)?;
        if b == 'B' {
            row |= 1 << i;
        }
        s = s1;
    }
    Ok((s, row))
}
fn parse_col(mut s: &str) -> IResult<&str, u8> {
    let mut col = 0;
    for i in (0..3).rev() {
        let (s1, b) = one_of("LR")(s)?;
        if b == 'R' {
            col |= 1 << i;
        }
        s = s1;
    }
    Ok((s, col))
}
fn parse_pass(s: &str) -> IResult<&str, (u8, u8)> {
    let (s, r) = parse_row(s)?;
    let (s, c) = parse_col(s)?;
    Ok((s, (r, c)))
}

fn parse_input(s: &str) -> IResult<&str, Vec<(u8, u8)>> {
    separated_list0(line_ending, parse_pass)(s)
}

trait Solution {
    fn part_1(&self) -> u16;
}
impl Solution for str {
    fn part_1(&self) -> u16 {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .iter()
            .map(|pass| pass.id())
            .max()
            .expect("Empty input")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(parse_pass("FBFBBFFRLR"), Ok(("", (44, 5))));
        assert_eq!((44, 5).id(), 357);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            parse_input(
                "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            ),
            Ok(("", vec![(70, 7), (14, 7), (102, 4)]))
        );
        assert_eq!(
            [(70, 7), (14, 7), (102, 4)]
                .iter()
                .map(|p| p.id())
                .collect::<Vec<_>>(),
            vec![567, 119, 820]
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_5").part_1(), 850);
    }
}
