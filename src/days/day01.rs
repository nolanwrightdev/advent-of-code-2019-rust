use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn hello(input: &str) -> Vec<u32> {
	input
		.lines()
		.map(|line| line.parse::<u32>().unwrap())
		.collect()
}

#[aoc(day1, part1)]
fn hallo(masses: &[u32]) -> u64 {
	masses
		.iter()
		.map(|x| (*x as f64 / 3f64).floor() as u64 - 2)
		.sum()
}
