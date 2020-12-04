/// Day 4
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char, one_of},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;

fn is_valid(passport: &[(&str, &str)]) -> bool {
    let fields: HashSet<_> = passport.iter().map(|(f, _)| f).collect();
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|f| fields.contains(f))
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

trait Solution {
    fn part_1(&self) -> usize;
}
impl Solution for str {
    fn part_1(&self) -> usize {
        parse_input(self)
            .expect("Failed to parse the input")
            .1
            .iter()
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
        assert!(is_valid(
            &parse_passport(
                "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm"
            )
            .unwrap()
            .1
        ));
        assert!(!is_valid(
            &parse_passport(
                "\
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929"
            )
            .unwrap()
            .1
        ));
        assert!(is_valid(
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
        assert!(!is_valid(
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
}
