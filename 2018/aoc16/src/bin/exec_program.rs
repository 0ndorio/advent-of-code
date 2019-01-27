use std::{
    io::{self, Read},
    str::FromStr,
};

use aoc16::{instruction::Instruction, operation::Executor, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let instructions: Result<Vec<Instruction>> = input.lines().map(Instruction::from_str).collect();
    let instructions = apply_opcode_mapping(instructions?);

    let state = Executor::default()
        .with_instructions(instructions)
        .run()
        .clone();

    println!("Final state of our program: {:?}", state);

    Ok(())
}

fn apply_opcode_mapping(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    instructions.iter_mut().for_each(|mut instruction| {
        instruction.opcode = match instruction.opcode {
            0 => 13,
            1 => 6,
            2 => 0,
            3 => 11,
            4 => 3,
            5 => 10,
            6 => 2,
            7 => 4,
            8 => 7,
            9 => 14,
            10 => 15,
            11 => 5,
            12 => 8,
            13 => 12,
            14 => 1,
            15 => 9,
            _ => panic!("Unknown opcode discovered: {:?}", instruction),
        };
    });

    instructions
}
