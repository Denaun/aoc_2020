//! Day 8

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, one_of},
    combinator::{all_consuming, map_res, recognize, value},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashSet;

pub type Instruction = (Op, i32);
pub type BootCode = Vec<Instruction>;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone)]
pub struct Runner {
    pub code: BootCode,
    pub ip: usize,
    pub acc: i32,
}
impl Runner {
    pub fn new(code: BootCode) -> Self {
        Self {
            code,
            ip: 0,
            acc: 0,
        }
    }
    pub fn is_finished(&self) -> bool {
        self.ip >= self.code.len()
    }
    pub fn execute_one(&mut self) -> &mut Self {
        match self.code[self.ip] {
            (Op::Acc, value) => {
                self.acc += value;
                self.ip += 1;
            }
            (Op::Jmp, offset) => {
                if offset >= 0 {
                    self.ip += offset as usize;
                } else {
                    self.ip -= (-offset) as usize;
                }
            }
            (Op::Nop, _) => self.ip += 1,
        }
        self
    }
}

fn find_loop(runner: &mut Runner) -> bool {
    let mut seen = HashSet::new();
    while !runner.is_finished() && !seen.contains(&runner.ip) {
        seen.insert(runner.ip);
        runner.execute_one();
    }
    !runner.is_finished()
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    separated_pair(
        alt((
            value(Op::Nop, tag("nop")),
            value(Op::Acc, tag("acc")),
            value(Op::Jmp, tag("jmp")),
        )),
        char(' '),
        map_res(recognize(preceded(one_of("+-"), digit1)), |s: &str| {
            s.parse()
        }),
    )(s)
}
fn parse_input(s: &str) -> IResult<&str, BootCode> {
    all_consuming(separated_list1(line_ending, parse_instruction))(s)
}

trait Solution {
    fn part_1(&self) -> i32;
}
impl Solution for str {
    fn part_1(&self) -> i32 {
        let mut runner = Runner::new(parse_input(self).expect("Failed to parse the input").1);
        if !find_loop(&mut runner) {
            panic!("No loop found");
        }
        runner.acc
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
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            Ok((
                "",
                vec![
                    (Op::Nop, 0),
                    (Op::Acc, 1),
                    (Op::Jmp, 4),
                    (Op::Acc, 3),
                    (Op::Jmp, -3),
                    (Op::Acc, -99),
                    (Op::Acc, 1),
                    (Op::Jmp, -4),
                    (Op::Acc, 6),
                ]
            ))
        )
    }

    #[test]
    fn example_1() {
        let mut runner = Runner::new(vec![
            (Op::Nop, 0),
            (Op::Acc, 1),
            (Op::Jmp, 4),
            (Op::Acc, 3),
            (Op::Jmp, -3),
            (Op::Acc, -99),
            (Op::Acc, 1),
            (Op::Jmp, -4),
            (Op::Acc, 6),
        ]);
        assert!(find_loop(&mut runner));
        assert_eq!(runner.ip, 1);
        assert_eq!(runner.acc, 5);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_8").part_1(), 2025);
    }
}
