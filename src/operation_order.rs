//! Day 18

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Op {
    Plus,
    Times,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<T> {
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

mod parsers {
    use std::str::FromStr;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, line_ending},
        combinator::{map, value},
        error::Error,
        multi::separated_list0,
        sequence::{delimited, pair, preceded},
        IResult,
    };

    use crate::parsers::*;

    use super::*;

    pub fn part_1<T: FromStr>(s: &str) -> Result<Vec<Expression<T>>, Error<&str>> {
        finished_parser(separated_list0(line_ending, basic_expression))(s)
    }
    pub fn basic_expression<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
        let (mut s, mut expr) = token(basic_expression)(s)?;
        while let Ok((s1, (op, lhs))) = pair(
            alt((value(Op::Plus, tag(" + ")), value(Op::Times, tag(" * ")))),
            token(basic_expression),
        )(s)
        {
            s = s1;
            expr = Expression::Op(Box::new(expr), op, Box::new(lhs));
        }
        Ok((s, expr))
    }

    fn token<T: FromStr>(
        parse_expression: impl FnMut(&str) -> IResult<&str, Expression<T>> + Copy,
    ) -> impl FnMut(&str) -> IResult<&str, Expression<T>> {
        move |s| {
            alt((
                map(integer, |v| Expression::Value(v)),
                delimited(char('('), parse_expression, char(')')),
            ))(s)
        }
    }

    pub fn part_2<T: FromStr>(s: &str) -> Result<Vec<Expression<T>>, Error<&str>> {
        finished_parser(separated_list0(line_ending, advanced_expression))(s)
    }
    pub fn advanced_expression<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
        let (mut s, mut expr) = high_precedence_operation(s)?;
        while let Ok((s1, lhs)) = preceded(tag(" * "), high_precedence_operation)(s) {
            s = s1;
            expr = Expression::Op(Box::new(expr), Op::Times, Box::new(lhs));
        }
        Ok((s, expr))
    }
    fn high_precedence_operation<T: FromStr>(s: &str) -> IResult<&str, Expression<T>> {
        let (mut s, mut expr) = token(advanced_expression)(s)?;
        while let Ok((s1, lhs)) = preceded(tag(" + "), token(advanced_expression))(s) {
            s = s1;
            expr = Expression::Op(Box::new(expr), Op::Plus, Box::new(lhs));
        }
        Ok((s, expr))
    }
}

trait Solution {
    fn part_1(&self) -> u64;
    fn part_2(&self) -> u64;
}
impl Solution for str {
    fn part_1(&self) -> u64 {
        parsers::part_1::<u64>(self)
            .expect("Failed to parse the input")
            .into_iter()
            .map(|expr| expr.evaluate())
            .sum()
    }
    fn part_2(&self) -> u64 {
        parsers::part_2::<u64>(self)
            .expect("Failed to parse the input")
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
            parsers::basic_expression("1 + 2 * 3 + 4 * 5 + 6"),
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
            parsers::basic_expression::<i32>("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .1
                .evaluate(),
            71
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            parsers::basic_expression::<i32>("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .1
                .evaluate(),
            51
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            parsers::basic_expression::<i32>("2 * 3 + (4 * 5)")
                .unwrap()
                .1
                .evaluate(),
            26
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            parsers::basic_expression::<i32>("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .1
                .evaluate(),
            437
        );
    }

    #[test]
    fn example_5() {
        assert_eq!(
            parsers::basic_expression::<i32>("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .1
                .evaluate(),
            12240
        );
    }

    #[test]
    fn example_6() {
        assert_eq!(
            parsers::basic_expression::<i32>("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
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
            parsers::advanced_expression::<i32>("1 + 2 * 3 + 4 * 5 + 6")
                .unwrap()
                .1
                .evaluate(),
            231
        );
    }

    #[test]
    fn example_8() {
        assert_eq!(
            parsers::advanced_expression::<i32>("1 + (2 * 3) + (4 * (5 + 6))")
                .unwrap()
                .1
                .evaluate(),
            51
        );
    }

    #[test]
    fn example_9() {
        assert_eq!(
            parsers::advanced_expression::<i32>("2 * 3 + (4 * 5)")
                .unwrap()
                .1
                .evaluate(),
            46
        );
    }

    #[test]
    fn example_10() {
        assert_eq!(
            parsers::advanced_expression::<i32>("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap()
                .1
                .evaluate(),
            1445
        );
    }

    #[test]
    fn example_11() {
        assert_eq!(
            parsers::advanced_expression::<i32>("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap()
                .1
                .evaluate(),
            669060
        );
    }

    #[test]
    fn example_12() {
        assert_eq!(
            parsers::advanced_expression::<i32>("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
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
