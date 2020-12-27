use std::str::FromStr;

use bitvec::prelude::*;
use nom::{
    branch::alt,
    character::complete::{char, digit1, line_ending, one_of},
    combinator::{all_consuming, map, map_res, recognize, value},
    error::ParseError,
    multi::{many1, separated_list1},
    sequence::{pair, preceded},
    Finish, IResult, InputLength, Parser,
};

pub fn finished_parser<I, O, E>(parser: impl Parser<I, O, E>) -> impl FnMut(I) -> Result<O, E>
where
    I: InputLength,
    E: ParseError<I>,
{
    let mut parser = all_consuming(parser);
    move |s| parser.parse(s).finish().map(|(_, v)| v)
}

pub fn number_list<T: FromStr>(s: &str) -> Result<Vec<T>, <T as FromStr>::Err> {
    s.lines().map(str::parse).collect()
}
pub fn integer<T: FromStr>(s: &str) -> IResult<&str, T> {
    map_res(digit1, |s: &str| s.parse())(s)
}
pub fn signed_integer<T: FromStr>(s: &str) -> IResult<&str, T> {
    map_res(recognize(preceded(one_of("+-"), digit1)), |s: &str| {
        s.parse()
    })(s)
}

pub fn bw_image(s: &str) -> IResult<&str, Vec<BitVec>> {
    separated_list1(
        line_ending,
        map(many1(bw_cell), |bits| bits.into_iter().collect()),
    )(s)
}
pub fn bw_cell(s: &str) -> IResult<&str, bool> {
    alt((value(true, char('#')), value(false, char('.'))))(s)
}

pub fn double_line_ending(s: &str) -> IResult<&str, &str> {
    recognize(pair(line_ending, line_ending))(s)
}
