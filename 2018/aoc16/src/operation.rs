#![allow(dead_code)]
use std::collections::VecDeque;

use crate::{instruction::Instruction, state::State};

#[derive(Debug, Clone)]
pub struct Executor {
    state: State,
    instructions: VecDeque<Instruction>,
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            state: Default::default(),
            instructions: Default::default(),
        }
    }
}

impl Executor {
    pub fn with_state(self, state: State) -> Self {
        let mut executor = Self::default();
        executor.state = state;

        executor
    }

    pub fn with_instruction(mut self, instruction: Instruction) -> Self {
        self.instructions.push_back(instruction);
        self
    }

    pub fn with_instructions<Instructions>(mut self, instructions: Instructions) -> Self
    where
        Instructions: IntoIterator<Item = Instruction>,
    {
        self.instructions.extend(instructions);
        self
    }

    pub fn run(&mut self) -> &State {
        while let Some(instruction) = self.instructions.pop_front() {
            self.exec(instruction);
        }

        &self.state
    }

    pub fn exec(&mut self, instruction: Instruction) {
        let lhs = instruction.input_lhs;
        let rhs = instruction.input_rhs;
        let target = instruction.output;

        let op = match instruction.opcode {
            0 => Addr::exec,
            1 => Addi::exec,
            2 => Mulr::exec,
            3 => Muli::exec,
            4 => Banr::exec,
            5 => Bani::exec,
            6 => Borr::exec,
            7 => Bori::exec,
            8 => Setr::exec,
            9 => Seti::exec,
            10 => Gtir::exec,
            11 => Gtri::exec,
            12 => Gtrr::exec,
            13 => Eqir::exec,
            14 => Eqri::exec,
            15 => Eqrr::exec,
            _ => panic!("Unknown opcode discovered: {:?}", instruction),
        };

        op(lhs, rhs, target, &mut self.state)
    }
}

trait Operation: Send + Sync {
    fn exec(a: u8, b: u8, c: u8, state: &mut State);
}

#[derive(Debug)]
struct Addr;

impl Operation for Addr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] + state[b];
    }
}

#[derive(Debug)]
struct Addi;

impl Operation for Addi {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] + u32::from(b);
    }
}

#[derive(Debug)]
struct Mulr;

impl Operation for Mulr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] * state[b]
    }
}

#[derive(Debug)]
struct Muli;

impl Operation for Muli {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] * u32::from(b);
    }
}

#[derive(Debug)]
struct Banr;

impl Operation for Banr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] & state[b];
    }
}

#[derive(Debug)]
struct Bani;

impl Operation for Bani {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] & u32::from(b);
    }
}

#[derive(Debug)]
struct Borr;

impl Operation for Borr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] | state[b];
    }
}

#[derive(Debug)]
struct Bori;

impl Operation for Bori {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = state[a] | u32::from(b);
    }
}

#[derive(Debug)]
struct Setr;

impl Operation for Setr {
    fn exec(a: u8, _b: u8, c: u8, state: &mut State) {
        state[c] = state[a];
    }
}

#[derive(Debug)]
struct Seti;

impl Operation for Seti {
    fn exec(a: u8, _b: u8, c: u8, state: &mut State) {
        state[c] = u32::from(a);
    }
}

#[derive(Debug)]
struct Gtir;

impl Operation for Gtir {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (u32::from(a) > state[b]) as u32;
    }
}

#[derive(Debug)]
struct Gtri;

impl Operation for Gtri {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (state[a] > u32::from(b)) as u32;
    }
}

#[derive(Debug)]
struct Gtrr;

impl Operation for Gtrr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (state[a] > state[b]) as u32;
    }
}

#[derive(Debug)]
struct Eqir;

impl Operation for Eqir {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (u32::from(a) == state[b]) as u32;
    }
}

#[derive(Debug)]
struct Eqri;

impl Operation for Eqri {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (state[a] == u32::from(b)) as u32;
    }
}

#[derive(Debug)]
struct Eqrr;

impl Operation for Eqrr {
    fn exec(a: u8, b: u8, c: u8, state: &mut State) {
        state[c] = (state[a] == state[b]) as u32;
    }
}
