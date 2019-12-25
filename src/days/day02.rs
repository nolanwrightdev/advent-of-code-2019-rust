use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn g(input: &str) -> Vec<usize> {
	input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
fn s1(instructions: &[usize]) -> usize {
	let mut memory = vec![0; instructions.len()];
	memory.copy_from_slice(instructions);
	memory[1] = 12;
	memory[2] = 2;
	execute_program(&mut memory)
}

#[aoc(day2, part2)]
fn s2(instructions: &[usize]) -> usize {
	let mut memory = vec![0; instructions.len()];
	memory.copy_from_slice(instructions);
	for noun in 0..100 {
		for verb in 0..100 {
			memory[1] = noun;
			memory[2] = verb;
			let result = execute_program(&mut memory);
			if result == 19690720 {
				return 100 * noun + verb;
			} else {
				memory.copy_from_slice(instructions);
			}
		}
	}
	unreachable!()
}

fn execute_program(memory: &mut [usize]) -> usize {
	let mut i = 0;
	loop {
		match memory[i] {
			1 => {
				let a = memory[i + 1];
				let b = memory[i + 2];
				let c = memory[i + 3];
				memory[c] = memory[a] + memory[b];
				i += 4;
			}
			2 => {
				let a = memory[i + 1];
				let b = memory[i + 2];
				let c = memory[i + 3];
				memory[c] = memory[a] * memory[b];
				i += 4;
			}
			99 => break,
			_ => panic!("invalid opcode"),
		}
	}
	memory[0]
}
