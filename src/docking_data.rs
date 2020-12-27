//! Day 14

use std::collections::HashMap;

use bitvec::prelude::*;
use itertools::Either;

trait Solution {
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        let mem = execute_v1(&parsers::input(self).expect("Failed to parse the input"));
        mem.into_iter().map(|(_, v)| v).sum()
    }
    fn part_2(&self) -> u64 {
        let mem = execute_v2(&parsers::input(self).expect("Failed to parse the input"));
        mem.into_iter().map(|(_, v)| v).sum()
    }
}

const N_BITS: usize = 36;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Mask {
    ones: u64,
    zeros: u64,
}
impl Default for Mask {
    fn default() -> Self {
        Self { ones: 0, zeros: 0 }
    }
}
impl Mask {
    fn get_floating_mask(&self) -> u64 {
        !(self.ones | self.zeros) & ((1 << N_BITS) - 1)
    }
}

fn execute_v1(instructions: &[Either<Mask, (u64, u64)>]) -> HashMap<u64, u64> {
    instructions
        .iter()
        .fold(
            (HashMap::new(), Mask::default()),
            |(mut mem, mask), instruction| match instruction {
                Either::Left(mask) => (mem, *mask),
                Either::Right((address, value)) => {
                    mem.insert(*address, (value & !mask.zeros) | mask.ones);
                    (mem, mask)
                }
            },
        )
        .0
}
fn execute_v2(instructions: &[Either<Mask, (u64, u64)>]) -> HashMap<u64, u64> {
    instructions
        .iter()
        .fold(
            (HashMap::new(), Mask::default()),
            |(mut mem, mask), instruction| match instruction {
                Either::Left(mask) => (mem, *mask),
                Either::Right((address, value)) => {
                    let floating_mask = mask.get_floating_mask();
                    let base_address = (address & !floating_mask) | mask.ones;
                    for fluctuation_ix in 0u32..1 << floating_mask.count_ones() {
                        let fluctuation: u64 = floating_mask
                            .view_bits::<Lsb0>()
                            .iter()
                            .enumerate()
                            .filter_map(|(ix, &b)| if b { Some(ix) } else { None })
                            .zip(fluctuation_ix.view_bits::<Lsb0>())
                            .filter_map(|(ix, &b)| if b { Some(1u64 << ix) } else { None })
                            .sum();
                        mem.insert(base_address | fluctuation, *value);
                    }
                    (mem, mask)
                }
            },
        )
        .0
}

mod parsers {
    use itertools::Either;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, one_of},
        combinator::map,
        error::Error,
        multi::{many_m_n, separated_list0},
        IResult,
    };

    use crate::parsers::{finished_parser, integer};

    use super::{Mask, N_BITS};

    pub fn input(s: &str) -> Result<Vec<Either<Mask, (u64, u64)>>, Error<&str>> {
        finished_parser(separated_list0(
            line_ending,
            alt((
                map(mask_line, |mask| Either::Left(mask)),
                map(mem_line, |mem| Either::Right(mem)),
            )),
        ))(s)
    }

    fn mask_line(s: &str) -> IResult<&str, Mask> {
        let (s, _) = tag("mask = ")(s)?;
        mask(s)
    }
    fn mask(s: &str) -> IResult<&str, Mask> {
        let (s, raw) = many_m_n(1, N_BITS, one_of("X01"))(s)?;
        Ok((
            s,
            Mask {
                ones: mask_for(&'1', &raw),
                zeros: mask_for(&'0', &raw),
            },
        ))
    }
    fn mask_for(x: &char, chars: &[char]) -> u64 {
        chars
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(ix, c)| if c == x { Some(1 << ix) } else { None })
            .sum()
    }

    fn mem_line(s: &str) -> IResult<&str, (u64, u64)> {
        let (s, _) = tag("mem[")(s)?;
        let (s, address) = integer(s)?;
        let (s, _) = tag("] = ")(s)?;
        let (s, value) = integer(s)?;
        Ok((s, (address, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parsers::input(
                "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"
            ),
            Ok(vec![
                Either::Left(Mask { ones: 64, zeros: 2 }),
                Either::Right((8, 11)),
                Either::Right((7, 101)),
                Either::Right((8, 0)),
            ])
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            execute_v1(&[
                Either::Left(Mask { ones: 64, zeros: 2 }),
                Either::Right((8, 11)),
                Either::Right((7, 101)),
                Either::Right((8, 0)),
            ]),
            [(7, 101), (8, 64)]
                .iter()
                .copied()
                .collect::<HashMap<_, _>>()
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_14").part_1(), 4_886_706_177_792);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            execute_v2(
                &parsers::input(
                    "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
                )
                .unwrap()
            ),
            [
                (26, 100),
                (27, 100),
                (58, 100),
                (59, 100),
                (16, 1),
                (17, 1),
                (18, 1),
                (19, 1),
                (24, 1),
                (25, 1),
                (26, 1),
                (27, 1),
            ]
            .iter()
            .copied()
            .collect::<HashMap<_, _>>()
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_14").part_2(), 3_348_493_585_827);
    }
}
