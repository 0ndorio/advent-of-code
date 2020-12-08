use std::convert::TryFrom;
use std::str::FromStr;
use std::{env, fs};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::{map, map_res, recognize};
use nom::multi::separated_list0;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let input_file = env::args().nth(1).expect("input file name missing");
    let file_content = fs::read_to_string(input_file)?;

    let bag_definitions = file_content
        .as_str()
        .lines()
        .filter_map(|input| Bag::try_from(input).ok())
        .collect::<Vec<_>>();

    let bags = BagDefinitions::from(bag_definitions.clone());
    let num_eventually_shiny_golden = bag_definitions
        .into_iter()
        .filter(|definition| bags.contains_shiny_golden(&definition.color))
        .count();

    println!("Num: {}", num_eventually_shiny_golden);
    Ok(())
}

// ------------------------------------------------------------------------------
// Bags
// ------------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct BagDefinitions(HashMap<BagColor, Bag>);

impl BagDefinitions {
    const SHINY_GOLDEN: &'static str = "shiny gold";

    fn contains_shiny_golden(&self, color: &BagColor) -> bool {
        let shiny_golden = BagColor(String::from(BagDefinitions::SHINY_GOLDEN));

        let mut missing = vec![color];

        while let Some(color) = missing.pop() {
            match self.0.get(color) {
                Some(definition) => {
                    let mut content_colors = definition
                        .content
                        .iter()
                        .map(|(_, color)| color)
                        .collect::<Vec<_>>();

                    if content_colors.contains(&&shiny_golden) {
                        return true;
                    }

                    missing.append(&mut content_colors)
                }
                None => continue,
            }
        }

        false
    }
}

impl From<Vec<Bag>> for BagDefinitions {
    fn from(definitions: Vec<Bag>) -> Self {
        let mut bags = HashMap::new();
        for definition in definitions {
            bags.insert(definition.color.clone(), definition);
        }

        Self(bags)
    }
}

// ------------------------------------------------------------------------------
// BagDefinition
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
struct Bag {
    color: BagColor,
    content: Vec<(u32, BagColor)>,
}

impl Bag {}

impl<'a> TryFrom<&'a str> for Bag {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        fn parse_bag_definition(input: &str) -> IResult<&str, Bag> {
            let amount = map_res(digit1, FromStr::from_str);

            let content = pair(amount, preceded(space1, BagColor::parse_color));
            let mut content = separated_list0(tag(", "), content);

            let (remaining, color) = BagColor::parse_color(input)?;
            let (remaining, _) = tag(" contain ")(remaining)?;
            let (remaining, content) = content(remaining)?;

            Ok((remaining, Bag { color, content }))
        }

        match parse_bag_definition(value) {
            Ok((_, bag_definition)) => Ok(bag_definition),
            _ => Err("Invalid Bag Definition".into()),
        }
    }
}

// ------------------------------------------------------------------------------
// BagColor
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Ord, PartialOrd, Eq, Hash, Clone)]
struct BagColor(String);

impl BagColor {
    fn parse_color(input: &str) -> IResult<&str, BagColor> {
        let color = recognize(delimited(alpha1, space1, alpha1));
        let color = terminated(color, alt((tag(" bags"), tag(" bag"))));

        let color = map(color, String::from);

        map(color, BagColor)(input)
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    const BAG_DEFINITIONS: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

    #[test]
    fn dummy() -> Result<(), Error> {
        let bag_definitions = BAG_DEFINITIONS
            .lines()
            .map(Bag::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(9, bag_definitions.len());

        let bags = BagDefinitions::from(bag_definitions.clone());
        let num_eventually_shiny_golden = bag_definitions
            .into_iter()
            .filter(|definition| bags.contains_shiny_golden(&definition.color))
            .count();

        assert_eq!(4, num_eventually_shiny_golden);

        Ok(())
    }
}
