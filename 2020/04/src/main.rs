use std::{env, fs};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::combinator::{map, opt, recognize};
use nom::multi::{many1, separated_list0};
use nom::sequence::preceded;
use nom::IResult;
use std::convert::TryFrom;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let input_file = env::args().nth(1).expect("input file name missing");
    let file_content = fs::read_to_string(input_file)?;

    let passports = file_content
        .as_str()
        .split("\n\n")
        .map(Passport::try_from)
        .collect::<Result<Vec<Passport>, _>>()?;

    println!("Num Passports: {}", passports.len());

    let num_valid_passports = passports.into_iter().filter(Passport::is_valid).count();

    println!("Num Valid Passports: {}", num_valid_passports);
    Ok(())
}

// ------------------------------------------------------------------------------
// Passport
// ------------------------------------------------------------------------------

#[derive(Debug, Default)]
struct Passport<'a> {
    birth_year: &'a str,
    country_id: &'a str,
    expiration_year: &'a str,
    eye_color: &'a str,
    hair_color: &'a str,
    height: &'a str,
    id: &'a str,
    issue_year: &'a str,
}

impl<'a> Passport<'a> {
    /// Check if the passport contains any default information.
    ///
    /// We are ignoring the `country_id` to ensure Northpole Credentials
    /// are treated as valid passports.
    fn is_valid(&self) -> bool {
        !(self.birth_year.is_empty()
            || self.expiration_year.is_empty()
            || self.eye_color.is_empty()
            || self.hair_color.is_empty()
            || self.height.is_empty()
            || self.id.is_empty()
            || self.issue_year.is_empty())
    }
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        fn parse_passport_entry(input: &str) -> IResult<&str, Vec<PassportEntry>> {
            let birth_year = map(preceded(tag("byr:"), digit1), PassportEntry::BirthYear);
            let issue_year = map(preceded(tag("iyr:"), digit1), PassportEntry::IssueYear);
            let expiration_year = map(preceded(tag("eyr:"), digit1), PassportEntry::ExpirationYear);
            let country_id = map(preceded(tag("cid:"), digit1), PassportEntry::CountryId);
            let height = map(preceded(tag("hgt:"), alphanumeric1), PassportEntry::Height);
            let eye_color = map(
                preceded(
                    tag("ecl:"),
                    recognize(preceded(opt(tag("#")), alphanumeric1)),
                ),
                PassportEntry::EyeColor,
            );
            let hair_color = map(
                preceded(
                    tag("hcl:"),
                    recognize(preceded(opt(tag("#")), alphanumeric1)),
                ),
                PassportEntry::HairColor,
            );
            let passport_id = map(
                preceded(tag("pid:"), preceded(opt(tag("#")), alphanumeric1)),
                PassportEntry::Id,
            );

            separated_list0(
                many1(alt((space1, tag("\n")))),
                alt((
                    birth_year,
                    country_id,
                    expiration_year,
                    eye_color,
                    hair_color,
                    height,
                    issue_year,
                    passport_id,
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

        Ok(passport)
    }
}

#[derive(Debug)]
enum PassportEntry<'a> {
    BirthYear(&'a str),
    CountryId(&'a str),
    IssueYear(&'a str),
    ExpirationYear(&'a str),
    EyeColor(&'a str),
    HairColor(&'a str),
    Height(&'a str),
    Id(&'a str),
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_passports() -> Result<(), Error> {
        let passports = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let num_valid_passports = passports
            .split("\n\n")
            .filter_map(|passport| Passport::try_from(passport).ok())
            .filter(Passport::is_valid)
            .count();

        assert_eq!(2, num_valid_passports);
        Ok(())
    }
}
