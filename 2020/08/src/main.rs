use std::convert::TryFrom;
use std::{env, fs};

use parse_display::{Display, FromStr};

fn main() -> Result<(), Error> {
    let instructions = parse_input::<Instruction>()?;
    let mut console = Console::from(&instructions);

    console.run_till_called_twice()?;
    println!(
        "Accu Before Instruction Called Twice: {}",
        console.state.accumulator
    );

    Ok(())
}

// ------------------------------------------------------------------------------
// Console
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Default)]
struct Console {
    instructions: Vec<(Instruction, u32)>,
    state: State,
}

impl Console {
    fn run_till_called_twice(&mut self) -> Result<(), Error> {
        let state = &mut self.state;

        loop {
            let offset = usize::try_from(state.offset).unwrap_or(usize::max_value());
            if offset >= self.instructions.len() {
                return Err("Offset Out-Of-Bounds Detected".into());
            }

            let instruction = &mut self.instructions[offset];

            if instruction.1 == 1 {
                return Ok(());
            }

            Console::run(state, instruction);
        }
    }

    fn run(state: &mut State, instruction: &mut (Instruction, u32)) {
        instruction.1 += 1;
        state.offset += 1;

        match instruction.0 {
            Instruction::Acc(value) => state.accumulator += value,
            Instruction::Jmp(value) => state.offset += value - 1,
            Instruction::Nop => {}
        }
    }
}

impl<InstructionSetT> From<InstructionSetT> for Console
where
    InstructionSetT: AsRef<[Instruction]>,
{
    fn from(instructions: InstructionSetT) -> Self {
        let instructions = instructions
            .as_ref()
            .iter()
            .map(|&instruction| (instruction, 0))
            .collect::<Vec<_>>();

        Self {
            instructions,
            ..Console::default()
        }
    }
}

// ------------------------------------------------------------------------------
// State
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Default, Copy, Clone)]
struct State {
    accumulator: i32,
    offset: i32,
}

// ------------------------------------------------------------------------------
// Instruction
// ------------------------------------------------------------------------------

#[derive(Debug, PartialEq, FromStr, Display, Copy, Clone)]
enum Instruction {
    #[display("acc {0}")]
    Acc(i32),
    #[display("jmp {0}")]
    Jmp(i32),
    #[display("nop")]
    #[from_str(regex = "nop .*")]
    Nop,
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

    #[test]
    fn run_test_program() -> Result<(), Error> {
        const TEST_PROGRAM: &str = "nop +0
                                acc +1
                                jmp +4
                                acc +3
                                jmp -3
                                acc -99
                                acc +1
                                jmp -4
                                acc +6";

        let instructions = TEST_PROGRAM
            .lines()
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<Vec<Instruction>, _>>()?;

        let mut console = Console::from(instructions);
        console.run_till_called_twice()?;

        assert_eq!(5, console.state.accumulator);
        Ok(())
    }
}
