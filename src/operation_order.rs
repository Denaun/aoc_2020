//! Day 1

use crate::docking_data::parse_integer;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending},
    combinator::{all_consuming, map, value},
    multi::separated_list0,
    sequence::{delimited, pair, preceded},
    IResult,
};
use std::{
    ops::{Add, Mul},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    Plus,
    Times,
}

#[derive(Debug, Clone, PartialEq)]
enum Expression<T> {
    Value(T),
    Op(Box<Expression<T>>, Op, Box<Expression<T>>),
}
impl<T: Add<Output = T> + Mul<Output = T>> Expression<T> {
    fn evaluate(self) -> T {
        match self {
            Expression::Value(v) => v,
            Expression::Op(lhs, op, rhs) => match op {
                Op::Plus => lhs.evaluate() + rhs.evaluate(),
                Op::Times => lhs.evaluate() * rhs.evaluate(),
            },
        }
    }
}

fn parse_part_1<T: FromStr>(s: &str) -> IResult<&str, Vec<Expression<T>>> {
    all_consuming(separated_list0(line_ending, parse_basic))(s)
}
fn parse_basic<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
    let (mut s, mut expr) = parse_token(parse_basic)(s)?;
    while let Ok((s1, (op, lhs))) = pair(
        alt((value(Op::Plus, tag(" + ")), value(Op::Times, tag(" * ")))),
        parse_token(parse_basic),
    )(s)
    {
        s = s1;
        expr = Expression::Op(Box::new(expr), op, Box::new(lhs));
    }
    Ok((s, expr))
}
fn parse_token<T: FromStr>(
    parse_expression: impl FnMut(&str) -> IResult<&str, Expression<T>> + Copy,
) -> impl FnMut(&str) -> IResult<&str, Expression<T>> {
    move |s| {
        alt((
            map(parse_integer, |v| Expression::Value(v)),
            delimited(char('('), parse_expression, char(')')),
        ))(s)
    }
}

fn parse_part_2<T: FromStr>(s: &str) -> IResult<&str, Vec<Expression<T>>> {
    all_consuming(separated_list0(line_ending, parse_advanced_low_precedence))(s)
}
fn parse_advanced_low_precedence<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
    let (mut s, mut expr) = parse_advanced_high_precedence(s)?;
    while let Ok((s1, lhs)) = preceded(tag(" * "), parse_advanced_high_precedence)(s) {
        s = s1;
        expr = Expression::Op(Box::new(expr), Op::Times, Box::new(lhs));
    }
    Ok((s, expr))
}
fn parse_advanced_high_precedence<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
    let (mut s, mut expr) = parse_token(parse_advanced_low_precedence)(s)?;
    while let Ok((s1, lhs)) = preceded(tag(" + "), parse_token(parse_advanced_low_precedence))(s) {
        s = s1;
        expr = Expression::Op(Box::new(expr), Op::Plus, Box::new(lhs));
    }
    Ok((s, expr))
}

trait Solution {
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        parse_part_1::<u64>(self)
            .expect("Failed to parse the input")
            .1
            .into_iter()
            .map(|expr| expr.evaluate())
            .sum()
    }
    fn part_2(&self) -> u64 {
        parse_part_2::<u64>(self)
            .expect("Failed to parse the input")
            .1
            .into_iter()
            .map(|expr| expr.evaluate())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        assert_eq!(
            parse_basic("1 + 2 * 3 + 4 * 5 + 6"),
            Ok((
                "",
                Expression::Op(
                    Box::new(Expression::Op(
                        Box::new(Expression::Op(
                            Box::new(Expression::Op(
                                Box::new(Expression::Op(
                                    Box::new(Expression::Value(1)),
                                    Op::Plus,
                                    Box::new(Expression::Value(2))
                                )),
                                Op::Times,
                                Box::new(Expression::Value(3))
                            )),
                            Op::Plus,
                            Box::new(Expression::Value(4))
                        )),
                        Op::Times,
                        Box::new(Expression::Value(5))
                    )),
                    Op::Plus,
                    Box::new(Expression::Value(6))
                )
            ))
        );
    }

    #[test]
    fn example_1() {
        assert_eq!(
            parse_basic::<i32>("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .1
                .evaluate(),
            71
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            parse_basic::<i32>("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .1
                .evaluate(),
            51
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            parse_basic::<i32>("2 * 3 + (4 * 5)").unwrap().1.evaluate(),
            26
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            parse_basic::<i32>("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .1
                .evaluate(),
            437
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            parse_basic::<i32>("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .1
                .evaluate(),
            12240
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            parse_basic::<i32>("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .unwrap()
                .1
                .evaluate(),
            13632
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_18").part_1(), 1_402_255_785_165);
    }

    #[test]
    fn example_7() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .1
                .evaluate(),
            231
        );
    }

    #[test]
    fn example_8() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .1
                .evaluate(),
            51
        );
    }

    #[test]
    fn example_9() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("2 * 3 + (4 * 5)")
                .unwrap()
                .1
                .evaluate(),
            46
        );
    }

    #[test]
    fn example_10() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .1
                .evaluate(),
            1445
        );
    }

    #[test]
    fn example_11() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .1
                .evaluate(),
            669060
        );
    }

    #[test]
    fn example_12() {
        assert_eq!(
            parse_advanced_low_precedence::<i32>("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .unwrap()
                .1
                .evaluate(),
            23340
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_18").part_2(), 119_224_703_255_966);
    }
}
