//! Day 8

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

fn find_loop(runner: &mut Runner) -> Vec<usize> {
    let mut seen = Vec::new();
    while !runner.is_finished() && !seen.contains(&runner.ip) {
        seen.push(runner.ip);
        runner.execute_one();
    }
    seen
}
fn fix_loop(runner: Runner) -> Option<Runner> {
    for i in find_loop(&mut runner.clone())
        .into_iter()
        .rev()
        .filter(|i| matches!(runner.code[*i].0, Op::Jmp | Op::Nop))
    {
        let mut runner = runner.clone();
        runner.code.get_mut(i).unwrap().0 = match runner.code[i].0 {
            Op::Jmp => Op::Nop,
            Op::Nop => Op::Jmp,
            _ => panic!(),
        };
        find_loop(&mut runner);
        if runner.is_finished() {
            return Some(runner);
        }
    }
    None
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending},
        combinator::value,
        error::Error,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use crate::parsers::{finished_parser, signed_integer};

    use super::{BootCode, Instruction, Op};

    pub fn input(s: &str) -> Result<BootCode, Error<&str>> {
        finished_parser(separated_list1(line_ending, instruction))(s)
    }
    fn instruction(s: &str) -> IResult<&str, Instruction> {
        separated_pair(
            alt((
                value(Op::Nop, tag("nop")),
                value(Op::Acc, tag("acc")),
                value(Op::Jmp, tag("jmp")),
            )),
            char(' '),
            signed_integer,
        )(s)
    }
}

trait Solution {
    fn part_1(&self) -> i32;
    fn part_2(&self) -> i32;
}
impl Solution for str {
    fn part_1(&self) -> i32 {
        let mut runner = Runner::new(parsers::input(self).expect("Failed to parse the input"));
        find_loop(&mut runner);
        if runner.is_finished() {
            panic!("No loop found");
        }
        runner.acc
    }
    fn part_2(&self) -> i32 {
        if let Some(runner) = fix_loop(Runner::new(
            parsers::input(self).expect("Failed to parse the input"),
        )) {
            runner.acc
        } else {
            panic!("Couldn't fix");
        }
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
            Ok(vec![
                (Op::Nop, 0),
                (Op::Acc, 1),
                (Op::Jmp, 4),
                (Op::Acc, 3),
                (Op::Jmp, -3),
                (Op::Acc, -99),
                (Op::Acc, 1),
                (Op::Jmp, -4),
                (Op::Acc, 6),
            ])
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
        assert_eq!(find_loop(&mut runner), vec![0, 1, 2, 6, 7, 3, 4]);
        assert_eq!(runner.ip, 1);
        assert_eq!(runner.acc, 5);
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_8").part_1(), 2025);
    }

    #[test]
    fn example_2() {
        let runner = Runner::new(vec![
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
        let runner = fix_loop(runner).unwrap();
        assert_eq!(runner.code[7].0, Op::Nop);
        assert_eq!(runner.acc, 8);
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_8").part_2(), 2001);
    }
}
