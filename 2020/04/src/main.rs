use std::{env, fs};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::character::complete::{digit1, space1};
use nom::character::is_hex_digit;
use nom::combinator::{map, map_res, recognize, verify};
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::convert::TryFrom;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let input_file = env::args().nth(1).expect("input file name missing");
    let file_content = fs::read_to_string(input_file)?;

    let passports = file_content
        .as_str()
        .split("\n\n")
        .filter_map(|input| Passport::try_from(input).ok())
        .count();

    println!("Num Valid Passports: {}", passports);
    Ok(())
}

// ------------------------------------------------------------------------------
// Passport
// ------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Passport<'a> {
    birth_year: u32,
    country_id: &'a str,
    expiration_year: u32,
    eye_color: &'a str,
    hair_color: &'a str,
    height: (u32, &'a str),
    id: &'a str,
    issue_year: u32,
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        fn parse_passport_entry(input: &str) -> IResult<&str, Vec<PassportEntry>> {
            separated_list0(
                many1(alt((space1, tag("\n")))),
                alt((
                    PassportEntry::parse_birth_year,
                    PassportEntry::parse_country_id,
                    PassportEntry::parse_expiration_year,
                    PassportEntry::parse_eye_color,
                    PassportEntry::parse_hair_color,
                    PassportEntry::parse_height,
                    PassportEntry::parse_issue_year,
                    PassportEntry::parse_passport_id,
                )),
            )(input)
        }

        let entries = parse_passport_entry(&value).unwrap_or_default().1;

        let mut passport = Passport::default();
        for entry in entries {
            match entry {
                PassportEntry::BirthYear(value) => passport.birth_year = value,
                PassportEntry::CountryId(value) => passport.country_id = value,
                PassportEntry::IssueYear(value) => passport.issue_year = value,
                PassportEntry::ExpirationYear(value) => passport.expiration_year = value,
                PassportEntry::EyeColor(value) => passport.eye_color = value,
                PassportEntry::HairColor(value) => passport.hair_color = value,
                PassportEntry::Height(value) => passport.height = value,
                PassportEntry::Id(value) => passport.id = value,
            }
        }

        if passport.birth_year == 0
            || passport.issue_year == 0
            || passport.expiration_year == 0
            || passport.eye_color.is_empty()
            || passport.hair_color.is_empty()
            || passport.height.0 == 0
            || passport.id.is_empty()
        {
            return Err("Missing Passport Data".into());
        }

        Ok(passport)
    }
}

#[derive(Debug)]
enum PassportEntry<'a> {
    BirthYear(u32),
    CountryId(&'a str),
    IssueYear(u32),
    ExpirationYear(u32),
    EyeColor(&'a str),
    HairColor(&'a str),
    Height((u32, &'a str)),
    Id(&'a str),
}

impl<'a> PassportEntry<'a> {
    fn parse_birth_year(input: &str) -> IResult<&str, Self> {
        let birth_year = preceded(tag("byr:"), verify(digit1, |s: &str| s.len() == 4));
        let birth_year = map_res(birth_year, str::parse::<u32>);
        let birth_year = verify(birth_year, |year| (1920..=2002).contains(year));

        map(birth_year, Self::BirthYear)(input)
    }

    fn parse_issue_year(input: &str) -> IResult<&str, Self> {
        let issue_year = preceded(tag("iyr:"), verify(digit1, |s: &str| s.len() == 4));
        let issue_year = map_res(issue_year, str::parse::<u32>);
        let issue_year = verify(issue_year, |year| (2010..=2020).contains(year));

        map(issue_year, Self::IssueYear)(input)
    }

    fn parse_expiration_year(input: &str) -> IResult<&str, Self> {
        let expiration_year = preceded(tag("eyr:"), verify(digit1, |s: &str| s.len() == 4));
        let expiration_year = map_res(expiration_year, str::parse::<u32>);
        let expiration_year = verify(expiration_year, |year| (2020..=2030).contains(year));

        map(expiration_year, Self::ExpirationYear)(input)
    }

    fn parse_height(input: &'a str) -> IResult<&'a str, Self> {
        let height = preceded(tag("hgt:"), tuple((digit1, alt((tag("cm"), tag("in"))))));
        let height = map_res(height, |(size, unit): (&str, &str)| {
            size.parse::<u32>().map(|size| (size, unit))
        });
        let height = verify(height, |(size, unit): &(u32, &str)| {
            if *unit == "cm" {
                (150..=193).contains(size)
            } else {
                (59..=76).contains(size)
            }
        });

        map(height, Self::Height)(input)
    }

    fn parse_hair_color(input: &'a str) -> IResult<&'a str, Self> {
        let hair_color = preceded(
            tag("hcl:"),
            recognize(preceded(
                tag("#"),
                take_while_m_n(6, 6, |c: char| is_hex_digit(c as u8)),
            )),
        );

        map(hair_color, PassportEntry::HairColor)(input)
    }

    fn parse_eye_color(input: &'a str) -> IResult<&'a str, Self> {
        let eye_color = preceded(
            tag("ecl:"),
            alt((
                tag("amb"),
                tag("blu"),
                tag("brn"),
                tag("gry"),
                tag("grn"),
                tag("hzl"),
                tag("oth"),
            )),
        );

        map(eye_color, PassportEntry::EyeColor)(input)
    }

    fn parse_passport_id(input: &'a str) -> IResult<&'a str, Self> {
        let passport_id = preceded(tag("pid:"), verify(digit1, |s: &str| s.len() == 9));
        map(passport_id, PassportEntry::Id)(input)
    }

    fn parse_country_id(input: &'a str) -> IResult<&'a str, Self> {
        map(preceded(tag("cid:"), digit1), PassportEntry::CountryId)(input)
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_passports() -> Result<(), Error> {
        let passports = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

        let num_valid_passports = passports
            .split("\n\n")
            .filter_map(|passport| Passport::try_from(passport).ok())
            .count();

        assert_eq!(4, num_valid_passports);
        Ok(())
    }

    #[test]
    fn check_invalid_passports_are_recognized() -> Result<(), Error> {
        let passports = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

        let num_valid_passports = passports
            .split("\n\n")
            .filter_map(|passports| Passport::try_from(passports).ok())
            .count();

        assert_eq!(0, num_valid_passports);
        Ok(())
    }
}
