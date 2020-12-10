use std::{env, fs};

fn main() -> Result<(), Error> {
    let adapters = parse_input::<u32>()?;

    let joltage_differences = PowerSupply::calc_joltage_differences(&adapters);
    println!("Joltage Differences: {:#?}", joltage_differences);

    Ok(())
}

// ------------------------------------------------------------------------------
// Adapter
// ------------------------------------------------------------------------------

struct PowerSupply {}

impl PowerSupply {
    fn find_device_joltage_rating(adapters: impl AsRef<[u32]>) -> u32 {
        match adapters.as_ref().iter().max() {
            Some(value) => value + 3,
            None => 0,
        }
    }

    fn calc_joltage_differences(unsorted_adapters: impl AsRef<[u32]>) -> (u32, u32, u32) {
        let device_joltage = Self::find_device_joltage_rating(&unsorted_adapters);

        let mut adapters = vec![0, device_joltage];
        adapters.extend(unsorted_adapters.as_ref());
        adapters.sort_unstable();

        let mut joltage_differences = (0, 0, 0);
        for index in 0..adapters.len() - 1 {
            let difference = adapters[index + 1] - adapters[index];
            match difference {
                1 => joltage_differences.0 += 1,
                2 => joltage_differences.1 += 1,
                3 => joltage_differences.2 += 1,
                _ => continue,
            }
        }

        joltage_differences
    }
}

// ------------------------------------------------------------------------------
// Utility
// ------------------------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

fn parse_input<ResultT>() -> Result<Vec<ResultT>, Error>
where
    ResultT: std::str::FromStr,
    ResultT::Err: Into<Error>,
{
    let input_file = env::args().nth(1).expect("input file name missing");

    fs::read_to_string(input_file)?
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_ADAPTER_BAG: &str = "16 10 15 5 1 11 7 19 6 12 4";
    const BIG_ADAPTER_BAG: &str =
        "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";

    #[test]
    fn find_adapter_chain_in_small_bag() -> Result<(), Error> {
        let adapters = SMALL_ADAPTER_BAG
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()?;

        let device_joltage = PowerSupply::find_device_joltage_rating(&adapters);
        assert_eq!(22, device_joltage);

        let joltage_differences = PowerSupply::calc_joltage_differences(&adapters);
        assert_eq!((7, 0, 5), joltage_differences);

        Ok(())
    }

    #[test]
    fn find_adapter_chain_in_big_bag() -> Result<(), Error> {
        let adapters = BIG_ADAPTER_BAG
            .split(' ')
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()?;

        let device_joltage = PowerSupply::find_device_joltage_rating(&adapters);
        assert_eq!(52, device_joltage);

        let joltage_differences = PowerSupply::calc_joltage_differences(&adapters);
        assert_eq!((22, 0, 10), joltage_differences);

        Ok(())
    }
}
