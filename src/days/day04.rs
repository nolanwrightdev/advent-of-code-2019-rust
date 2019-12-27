use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;

#[aoc_generator(day4)]
fn g(input: &str) -> Range<u32> {
	let mut iter = input.split('-');
	let start = iter.next().unwrap().parse::<u32>().unwrap();
	let end = iter.next().unwrap().parse::<u32>().unwrap();
	Range { start, end }
}

#[aoc(day4, part1)]
fn s1(range: &Range<u32>) -> u32 {
	let mut matches = 0;
	for i in range.clone() {
		let digits = i.to_string();
		if digits.len() == 6 {
			let mut iter = digits.chars();
			let mut last = iter.next().unwrap();
			let mut adjacent_repitition = false;
			let mut never_decrease = true;
			for digit in iter {
				if digit == last {
					adjacent_repitition = true;
				} else if digit < last {
					never_decrease = false;
					break;
				}
				last = digit;
			}
			if adjacent_repitition && never_decrease {
				matches += 1;
			}
		}
	}
	matches
}

#[aoc(day4, part2)]
fn s2(range: &Range<u32>) -> u32 {
	let mut matches = 0;
	for i in range.clone() {
		let digits = i.to_string();
		if digits.len() == 6 {
			let mut iter = digits.chars();
			let mut last = iter.next().unwrap();
			let mut adjacent_repitition_count = 1;
			let mut adjacent_repitition = false;
			let mut never_decrease = true;
			for digit in iter {
				if digit == last {
					adjacent_repitition_count += 1;
				} else if digit < last {
					never_decrease = false;
					break;
				} else {
					if adjacent_repitition_count == 2 {
						adjacent_repitition = true;
					}
					adjacent_repitition_count = 1;
				}
				last = digit;
			}
			if adjacent_repitition_count == 2 {
				adjacent_repitition = true;
			}
			if adjacent_repitition && never_decrease {
				matches += 1;
			}
		}
	}
	matches
}
