use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

enum ProgramState<'a> {
	AwaitingInput(Program<'a>),
	Finished(Vec<i32>),
}

struct Program<'a> {
	pointer: usize,
	memory: &'a mut [i32],
	output: Vec<i32>,
}

impl<'a> Program<'a> {
	fn new(memory: &'a mut [i32]) -> Self {
		Program {
			memory,
			pointer: 0,
			output: Vec::new(),
		}
	}

	fn read_output(&mut self) -> Vec<i32> {
		std::mem::replace(&mut self.output, Vec::new())
	}

	fn execute(mut self, mut input: Option<i32>) -> ProgramState<'a> {
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

		loop {
			match decode_instruction(self.pointer, self.memory) {
				Instruction::Add(a, b, c) => {
					self.memory[c] = a + b;
					self.pointer += 4;
				}
				Instruction::Multiply(a, b, c) => {
					self.memory[c] = a * b;
					self.pointer += 4;
				}
				Instruction::Input(a) => match input.take() {
					Some(s) => {
						self.memory[a] = s;
						self.pointer += 2;
					}
					None => {
						return ProgramState::AwaitingInput(self);
					}
				},
				Instruction::Output(a) => {
					self.output.push(a);
					self.pointer += 2;
				}
				Instruction::JumpIfTrue(a, b) => {
					self.pointer = if a != 0 { b } else { self.pointer + 3 };
				}
				Instruction::JumpIfFalse(a, b) => {
					self.pointer = if a == 0 { b } else { self.pointer + 3 };
				}
				Instruction::LessThan(a, b, c) => {
					self.memory[c] = if a < b { 1 } else { 0 };
					self.pointer += 4;
				}
				Instruction::Equals(a, b, c) => {
					self.memory[c] = if a == b { 1 } else { 0 };
					self.pointer += 4;
				}
				Instruction::Halt => break,
			}
		}

		ProgramState::Finished(self.output)
	}
}

fn generate_combinations(choices: [u8; 5]) -> Vec<Vec<u8>> {
	let mut combinations = Vec::with_capacity(5 * 4 * 3 * 2);
	let mut queue = vec![(Vec::with_capacity(5), choices.to_vec())];
	while let Some((a, b)) = queue.pop() {
		if b.is_empty() {
			combinations.push(a);
		} else {
			for (i, &choice) in b.iter().enumerate() {
				let mut selected = a.clone();
				let mut choices = b.clone();
				choices.remove(i);
				selected.push(choice);
				queue.push((selected, choices));
			}
		}
	}
	combinations
}

#[aoc_generator(day07)]
fn g(input: &str) -> Vec<i32> {
	input
		.split(',')
		.map(|x| x.parse::<i32>().unwrap())
		.collect()
}

#[aoc(day07, part1)]
fn s1(program: &[i32]) -> i32 {
	let mut memory = vec![0; program.len()];
	let mut max_output = std::i32::MIN;
	for comb in generate_combinations([0, 1, 2, 3, 4]) {
		let mut x = 0;
		for phase_setting in comb {
			memory.copy_from_slice(program);
			let mut program = Program::new(&mut memory);
			match program.execute(Some(phase_setting as i32)) {
				ProgramState::AwaitingInput(p) => {
					program = p;
				}
				_ => panic!("program exected to wait for input"),
			}
			match program.execute(Some(x)) {
				ProgramState::Finished(v) => {
					x = v[0];
				}
				_ => panic!("program expected to be finished"),
			}
		}
		if x > max_output {
			max_output = x;
		}
	}
	max_output
}

#[aoc(day07, part2)]
fn s2(program_instructions: &[i32]) -> i32 {
	let mut max_output = std::i32::MIN;
	for combination in generate_combinations([5, 6, 7, 8, 9]) {
		// I shouldn't have to reinitialize these every cycle,
		// but sadly I could not overcome the borrow checker.
		let mut memory = vec![vec![0; program_instructions.len()]; 5];
		let mut states = Vec::with_capacity(5);
		for (i, m) in memory.iter_mut().enumerate() {
			m.copy_from_slice(program_instructions);
			states.push(Program::new(m).execute(Some(combination[i] as i32)));
		}
		let mut state_i = 0;
		let mut previous_output = vec![0];
		let mut break_flag = false;
		while !break_flag {
			loop {
				let state = &mut states[state_i];
				match state {
					ProgramState::AwaitingInput(program) => {
						if previous_output.is_empty() {
							state_i = (state_i + 1) % 5;
							previous_output = program.read_output();
							previous_output.reverse();
						} else {
							// This is very hacky.
							let program_rep = std::mem::replace(
								state,
								ProgramState::Finished(Vec::new()),
							);
							if let ProgramState::AwaitingInput(program) =
								program_rep
							{
								*state = program.execute(previous_output.pop());
							}
						}
					}
					ProgramState::Finished(output) => {
						output.reverse();
						previous_output = output.clone();
						if state_i == 4 {
							break_flag = true;
							break;
						} else {
							state_i = (state_i + 1) % 5;
						}
					}
				}
			}
		}
		let output = previous_output.pop().unwrap();
		if output > max_output {
			max_output = output;
		}
	}
	max_output
}
