use std::{error::Error, str::FromStr};

use crate::Result;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub input_lhs: u8,
    pub input_rhs: u8,
    pub output: u8,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Self> {
        let parsed: Vec<u8> = input.split_whitespace().flat_map(u8::from_str).collect();
        if parsed.len() != 4 {
            return Err(format!("Malformed instruction discovered: {}", input))?;
        }

        let opcode = parsed[0];
        let input_lhs = parsed[1];
        let input_rhs = parsed[2];
        let output = parsed[3];

        Ok(Self {
            opcode,
            input_lhs,
            input_rhs,
            output,
        })
    }
}
