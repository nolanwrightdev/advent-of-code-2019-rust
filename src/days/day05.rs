use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

fn execute_program(memory: &mut [i32], input: i32) -> Vec<i32> {
	const ADD_OP: i32 = 1;
	const MULT_OP: i32 = 2;
	const IN_OP: i32 = 3;
	const OUT_OP: i32 = 4;
	const JIT_OP: i32 = 5;
	const JIF_OP: i32 = 6;
	const LT_OP: i32 = 7;
	const EQ_OP: i32 = 8;
	const HALT_OP: i32 = 99;
	const POSITION_MODE: i32 = 0;
	const IMMEDIATE_MODE: i32 = 1;

	enum Instruction {
		Add(i32, i32, usize),
		Multiply(i32, i32, usize),
		Input(usize),
		Output(i32),
		JumpIfTrue(i32, usize),
		JumpIfFalse(i32, usize),
		LessThan(i32, i32, usize),
		Equals(i32, i32, usize),
		Halt,
	}

	enum Mode {
		Position,
		Immediate,
	}

	impl TryFrom<i32> for Mode {
		type Error = &'static str;
		fn try_from(value: i32) -> Result<Self, Self::Error> {
			match value {
				POSITION_MODE => Ok(Mode::Position),
				IMMEDIATE_MODE => Ok(Mode::Immediate),
				_ => Err("unsupported parameter mode"),
			}
		}
	}

	fn fetch_parameter(pointer: usize, memory: &[i32], mode: Mode) -> i32 {
		match mode {
			Mode::Position => memory[memory[pointer] as usize],
			Mode::Immediate => memory[pointer],
		}
	}

	fn decode_instruction(
		instruction_pointer: usize,
		memory: &[i32],
	) -> Instruction {
		let opcode = memory[instruction_pointer];
		match opcode % 100 {
			ADD_OP => Instruction::Add(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				),
				memory[instruction_pointer + 3 as usize] as usize,
			),
			MULT_OP => Instruction::Multiply(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				),
				memory[instruction_pointer + 3 as usize] as usize,
			),
			IN_OP => Instruction::Input(
				memory[instruction_pointer + 1 as usize] as usize,
			),
			OUT_OP => Instruction::Output(fetch_parameter(
				instruction_pointer + 1,
				memory,
				Mode::try_from((opcode / 100) % 10).unwrap(),
			)),
			JIT_OP => Instruction::JumpIfTrue(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				) as usize,
			),
			JIF_OP => Instruction::JumpIfFalse(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				) as usize,
			),
			LT_OP => Instruction::LessThan(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				),
				memory[instruction_pointer + 3 as usize] as usize,
			),
			EQ_OP => Instruction::Equals(
				fetch_parameter(
					instruction_pointer + 1,
					memory,
					Mode::try_from((opcode / 100) % 10).unwrap(),
				),
				fetch_parameter(
					instruction_pointer + 2,
					memory,
					Mode::try_from((opcode / 1000) % 10).unwrap(),
				),
				memory[instruction_pointer + 3 as usize] as usize,
			),
			HALT_OP => Instruction::Halt,
			_ => panic!("unsupported opcode"),
		}
	}

	let mut i: usize = 0;
	let mut output = Vec::new();

	loop {
		match decode_instruction(i, memory) {
			Instruction::Add(a, b, c) => {
				memory[c] = a + b;
				i += 4;
			}
			Instruction::Multiply(a, b, c) => {
				memory[c] = a * b;
				i += 4;
			}
			Instruction::Input(a) => {
				memory[a] = input;
				i += 2;
			}
			Instruction::Output(a) => {
				output.push(a);
				i += 2;
			}
			Instruction::JumpIfTrue(a, b) => i = if a != 0 { b } else { i + 3 },
			Instruction::JumpIfFalse(a, b) => {
				i = if a == 0 { b } else { i + 3 }
			}
			Instruction::LessThan(a, b, c) => {
				memory[c] = if a < b { 1 } else { 0 };
				i += 4;
			}
			Instruction::Equals(a, b, c) => {
				memory[c] = if a == b { 1 } else { 0 };
				i += 4;
			}
			Instruction::Halt => break,
		}
	}

	output
}

#[aoc_generator(day5)]
fn g(input: &str) -> Vec<i32> {
	input
		.split(',')
		.map(|x| x.parse::<i32>().unwrap())
		.collect()
}

#[aoc(day5, part1)]
fn s1(program: &[i32]) -> i32 {
	let mut memory = vec![0; program.len()];
	memory.copy_from_slice(program);
	let output = execute_program(&mut memory, 1);
	*output.last().unwrap()
}

#[aoc(day5, part2)]
fn s2(program: &[i32]) -> i32 {
	let mut memory = vec![0; program.len()];
	memory.copy_from_slice(program);
	let output = execute_program(&mut memory, 5);
	*output.last().unwrap()
}
