//! Day 4

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while_m_n},
    character::complete::{alpha1, char, digit1, one_of},
    combinator::{map_res, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::{HashMap, HashSet};

fn is_roughly_valid(passport: &[(&str, &str)]) -> bool {
    let fields: HashSet<_> = passport.iter().map(|(f, _)| f).collect();
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|f| fields.contains(f))
}

fn is_valid(passport: &HashMap<&str, &str>) -> bool {
    passport.get("byr").filter(|v| byr_valid(v)).is_some()
        && passport.get("iyr").filter(|v| iyr_valid(v)).is_some()
        && passport.get("eyr").filter(|v| eyr_valid(v)).is_some()
        && passport.get("hgt").filter(|v| hgt_valid(v)).is_some()
        && passport.get("hcl").filter(|v| hcl_valid(v)).is_some()
        && passport.get("ecl").filter(|v| ecl_valid(v)).is_some()
        && passport.get("pid").filter(|v| pid_valid(v)).is_some()
}

fn parse_passport(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        one_of(" \n"),
        separated_pair(alpha1, char(':'), is_not(" \n")),
    )(s)
}
fn parse_input(s: &str) -> IResult<&str, Vec<Vec<(&str, &str)>>> {
    separated_list1(tag("\n\n"), parse_passport)(s)
}

fn byr_valid(v: &str) -> bool {
    v.parse::<u32>()
        .ok()
        .filter(|&v| v >= 1920 && v <= 2002)
        .is_some()
}
fn iyr_valid(v: &str) -> bool {
    v.parse::<u32>()
        .ok()
        .filter(|&v| v >= 2010 && v <= 2020)
        .is_some()
}
fn eyr_valid(v: &str) -> bool {
    v.parse::<u32>()
        .ok()
        .filter(|&v| v >= 2020 && v <= 2030)
        .is_some()
}
fn hgt_valid(v: &str) -> bool {
    height(v)
        .ok()
        .filter(|(s, h)| {
            s == &""
                && match h {
                    Height::Cm(h) => *h >= 150 && *h <= 193,
                    Height::In(h) => *h >= 59 && *h <= 76,
                }
        })
        .is_some()
}
fn hcl_valid(v: &str) -> bool {
    matches!(hex_color(v), Ok(("", _)))
}
fn ecl_valid(v: &str) -> bool {
    matches!(v, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}
fn pid_valid(v: &str) -> bool {
    v.len() == 9 && v.chars().all(|c| c.is_digit(10))
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Height {
    Cm(u32),
    In(u32),
}
fn height(s: &str) -> IResult<&str, Height> {
    let (s, h) = map_res(digit1, |s: &str| s.parse())(s)?;
    alt((
        value(Height::Cm(h), tag("cm")),
        value(Height::In(h), tag("in")),
    ))(s)
}

fn hex_color(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("#")(s)?;
    take_while_m_n(6, 6, |c: char| c.is_digit(16))(s)
}

trait Solution {
    fn part_1(&self) -> usize;
    fn part_2(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .iter()
            .filter(|passport| is_roughly_valid(passport))
            .count()
    }
    fn part_2(&self) -> usize {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .into_iter()
            .map(|passport| passport.into_iter().collect())
            .filter(|passport| is_valid(passport))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_passport() {
        assert_eq!(
            parse_passport(
                "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
            ),
            Ok((
                "",
                vec![
                    ("ecl", "gry"),
                    ("pid", "860033327"),
                    ("eyr", "2020"),
                    ("hcl", "#fffffd"),
                    ("byr", "1937"),
                    ("iyr", "2017"),
                    ("cid", "147"),
                    ("hgt", "183cm")
                ]
            ))
        );
    }

    #[test]
    fn example_input() {
        assert_eq!(
            parse_input(
                "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            ),
            Ok((
                "",
                vec![
                    vec![
                        ("ecl", "gry"),
                        ("pid", "860033327"),
                        ("eyr", "2020"),
                        ("hcl", "#fffffd"),
                        ("byr", "1937"),
                        ("iyr", "2017"),
                        ("cid", "147"),
                        ("hgt", "183cm")
                    ],
                    vec![
                        ("iyr", "2013"),
                        ("ecl", "amb"),
                        ("cid", "350"),
                        ("eyr", "2023"),
                        ("pid", "028048884"),
                        ("hcl", "#cfa07d"),
                        ("byr", "1929"),
                    ],
                    vec![
                        ("hcl", "#ae17e1"),
                        ("iyr", "2013"),
                        ("eyr", "2024"),
                        ("ecl", "brn"),
                        ("pid", "760753108"),
                        ("byr", "1931"),
                        ("hgt", "179cm"),
                    ],
                    vec![
                        ("hcl", "#cfa07d"),
                        ("eyr", "2025"),
                        ("pid", "166559648"),
                        ("iyr", "2011"),
                        ("ecl", "brn"),
                        ("hgt", "59in"),
                    ],
                ]
            ))
        );
    }

    #[test]
    fn example_1() {
        assert!(is_roughly_valid(
            &parse_passport(
                "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
            )
            .unwrap()
            .1
        ));
        assert!(!is_roughly_valid(
            &parse_passport(
                "\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
            )
            .unwrap()
            .1
        ));
        assert!(is_roughly_valid(
            &parse_passport(
                "\
hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm"
            )
            .unwrap()
            .1
        ));
        assert!(!is_roughly_valid(
            &parse_passport(
                "\
hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
            )
            .unwrap()
            .1
        ));
    }

    #[test]
    fn part_1() {
        assert_eq!(include_str!("inputs/day_4").part_1(), 260);
    }

    #[test]
    fn example_2() {
        assert!(byr_valid("2002"));
        assert!(!byr_valid("2003"));
        assert!(hgt_valid("60in"));
        assert!(hgt_valid("190cm"));
        assert!(!hgt_valid("190in"));
        assert!(!hgt_valid("190"));
        assert!(hcl_valid("#123abc"));
        assert!(!hcl_valid("#123abz"));
        assert!(!hcl_valid("123abc"));
        assert!(ecl_valid("brn"));
        assert!(!ecl_valid("wat"));
        assert!(pid_valid("000000001"));
        assert!(!pid_valid("0123456789"));
    }

    #[test]
    fn example_3() {
        assert!(parse_input(
            "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
        )
        .unwrap()
        .1
        .into_iter()
        .map(|passport| passport.into_iter().collect::<HashMap<_, _>>())
        .all(|passport| !is_valid(&passport)));
    }

    #[test]
    fn example_4() {
        assert!(parse_input(
            "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )
        .unwrap()
        .1
        .into_iter()
        .map(|passport| passport.into_iter().collect::<HashMap<_, _>>())
        .all(|passport| is_valid(&passport)));
    }

    #[test]
    fn part_2() {
        assert_eq!(include_str!("inputs/day_4").part_2(), 153);
    }
}
