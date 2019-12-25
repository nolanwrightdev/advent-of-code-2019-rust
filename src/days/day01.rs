use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn g(input: &str) -> Vec<i32> {
	input
		.lines()
		.map(|line| line.parse::<i32>().unwrap())
		.collect()
}

#[aoc(day1, part1)]
fn s1(masses: &[i32]) -> i32 {
	masses.iter().map(calculate_fuel_requirement).sum()
}

#[aoc(day1, part2)]
fn s2(masses: &[i32]) -> i32 {
	masses
		.iter()
		.map(|mass| {
			let mut fuel_requirement = calculate_fuel_requirement(mass);
			let mut total_fuel_required = fuel_requirement;
			loop {
				fuel_requirement =
					calculate_fuel_requirement(&fuel_requirement);
				if fuel_requirement > 0 {
					total_fuel_required += fuel_requirement;
				} else {
					return total_fuel_required;
				}
			}
		})
		.sum()
}

fn calculate_fuel_requirement(mass: &i32) -> i32 {
	(*mass as f64 / 3f64).floor() as i32 - 2
}
