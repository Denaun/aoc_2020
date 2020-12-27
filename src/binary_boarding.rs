//! Day 5

trait Solution {
    fn part_1(&self) -> u16;
    fn part_2(&self) -> u16;
}
impl Solution for str {
    fn part_1(&self) -> u16 {
        parsers::input(self)
            .expect("Failed to parse the input")
            .into_iter()
            .map(|pass| pass.id())
            .max()
            .expect("Empty input")
    }
    fn part_2(&self) -> u16 {
        let mut ids = parsers::input(self)
            .expect("Failed to parse the input")
            .into_iter()
            .map(|pass| pass.id())
            .collect::<Vec<_>>();
        ids.sort();
        ids.iter()
            .zip(ids.iter().skip(1))
            .find_map(|(&a, &b)| if b == a + 2 { Some(a + 1) } else { None })
            .expect("Seat not found")
    }
}

pub trait Pass {
    fn id(&self) -> u16;
}
impl Pass for (u8, u8) {
    fn id(&self) -> u16 {
        self.0 as u16 * 8 + self.1 as u16
    }
}

mod parsers {
    use nom::{
        character::complete::{line_ending, one_of},
        error::Error,
        multi::separated_list0,
        sequence::pair,
        IResult,
    };

    use crate::parsers::finished_parser;

    use super::Pass;

    pub fn input(s: &str) -> Result<Vec<impl Pass>, Error<&str>> {
        finished_parser(separated_list0(line_ending, pass))(s)
    }
    pub fn pass(s: &str) -> IResult<&str, (u8, u8)> {
        pair(row, col)(s)
    }

    fn row(mut s: &str) -> IResult<&str, u8> {
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
    fn col(mut s: &str) -> IResult<&str, u8> {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(parsers::pass("FBFBBFFRLR"), Ok(("", (44, 5))));
        assert_eq!((44, 5).id(), 357);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            parsers::input(
                "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"
            )
            .unwrap()
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

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_5").part_2(), 599);
    }
}
