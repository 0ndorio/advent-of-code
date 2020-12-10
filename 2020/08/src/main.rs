use std::convert::TryFrom;
use std::{env, fs};

use parse_display::{Display, FromStr};
use std::cmp::Ordering;

fn main() -> Result<(), Error> {
    let instructions = parse_input::<Instruction>()?;
    let mut console = Console::from(&instructions);

    console.run_till_max_loop_depth(2)?;
    println!(
        "Accu Before Instruction Called Twice: {}",
        console.state.accumulator
    );

    if let Some(state) = console.run_with_error_correction(5)? {
        println!("Accu After Final Instruction: {}", state.accumulator);
    } else {
        println!("Couldn't detect fixed version.");
    }

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
    fn run(
        &mut self,
        callback: impl Fn(&State, &(Instruction, u32)) -> bool,
    ) -> Result<Option<State>, Error> {
        let state = &mut self.state;
        let num_instructions = self.instructions.len();

        loop {
            let offset = usize::try_from(state.offset).unwrap_or(usize::max_value());
            match offset.cmp(&num_instructions) {
                Ordering::Equal => return Ok(Some(*state)),
                Ordering::Greater => return Err("Offset Out-Of-Bounds Detected".into()),
                Ordering::Less => {}
            }

            let instruction = &mut self.instructions[offset];
            if !callback(state, instruction) {
                return Ok(None);
            }

            state.update(instruction);
        }
    }

    fn run_till_max_loop_depth(&mut self, loop_depth: u32) -> Result<Option<State>, Error> {
        self.run(|_, instruction| {
            if instruction.1 == loop_depth - 1 {
                return false;
            }

            true
        })
    }

    fn run_with_error_correction(&mut self, max_loop_depth: u32) -> Result<Option<State>, Error> {
        let original_instructions = self.instructions.clone();

        for index in 0..self.instructions.len() {
            self.reset(original_instructions.clone());

            self.instructions[index].0 = match self.instructions[index].0 {
                Instruction::Acc(_) => continue,
                Instruction::Jmp(value) => Instruction::Nop(value),
                Instruction::Nop(value) => Instruction::Jmp(value),
            };

            if let Some(state) = self.run_till_max_loop_depth(max_loop_depth)? {
                return Ok(Some(state));
            }
        }

        Ok(None)
    }

    fn reset(&mut self, instructions: Vec<(Instruction, u32)>) {
        self.state = State::default();
        self.instructions = instructions;
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

impl State {
    fn update(&mut self, instruction: &mut (Instruction, u32)) {
        instruction.1 += 1;
        self.offset += 1;

        match instruction.0 {
            Instruction::Acc(value) => self.accumulator += value,
            Instruction::Jmp(value) => self.offset += value - 1,
            Instruction::Nop(_) => {}
        }
    }
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
    #[display("nop {0}")]
    Nop(i32),
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
    fn run_test_program_abort_first_loop() -> Result<(), Error> {
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
        console.run_till_max_loop_depth(2)?;

        assert_eq!(5, console.state.accumulator);
        Ok(())
    }

    #[test]
    fn run_test_program_fix_endless_loop() -> Result<(), Error> {
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
        console.run_with_error_correction(2)?;

        assert_eq!(8, console.state.accumulator);
        Ok(())
    }
}
