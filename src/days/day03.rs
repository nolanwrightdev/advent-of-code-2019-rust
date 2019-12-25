use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
enum Direction {
	Left,
	Right,
	Up,
	Down,
}

struct Segment {
	direction: Direction,
	distance: u32,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);

impl Point {
	fn translate(&self, direction: Direction, distance: u32) -> Point {
		match direction {
			Direction::Left => Point(self.0 - distance as i32, self.1),
			Direction::Right => Point(self.0 + distance as i32, self.1),
			Direction::Up => Point(self.0, self.1 + distance as i32),
			Direction::Down => Point(self.0, self.1 - distance as i32),
		}
	}
	fn manhattan_distance_from_origin(&self) -> i32 {
		self.0 + self.1
	}
}

#[derive(Clone)]
struct Counted<T> {
	x: T,
	count: u32,
}

impl<T> PartialEq for Counted<T>
where
	T: PartialEq,
{
	fn eq(&self, other: &Self) -> bool {
		self.x.eq(&other.x)
	}
}

impl<T> Eq for Counted<T> where T: PartialEq {}

impl<T> Hash for Counted<T>
where
	T: Hash,
{
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.x.hash(state);
	}
}

type Path = Vec<Segment>;

type PointsVisited = HashSet<Point>;

#[aoc_generator(day3)]
fn g(input: &str) -> Vec<Path> {
	let mut paths = Vec::with_capacity(2);
	for line in input.lines() {
		let codes = line.split(',');
		let mut path = Vec::new();
		for code in codes {
			let mut chars = code.chars();
			let direction = match chars.next().unwrap() {
				'L' => Direction::Left,
				'R' => Direction::Right,
				'U' => Direction::Up,
				'D' => Direction::Down,
				_ => panic!("invalid character"),
			};
			let distance = chars.as_str().parse().unwrap();
			path.push(Segment {
				direction,
				distance,
			});
		}
		paths.push(path);
	}
	paths
}

#[aoc(day3, part1)]
fn s1(paths: &[Path]) -> i32 {
	// It would have been more efficient to only create one PointsVisited object
	// and then simply query it when going through the second path, maintaining
	// a state variable to keep track of the closest intersection point.
	let mut tours: Vec<PointsVisited> = Vec::with_capacity(2);
	for path in paths {
		let mut tour: PointsVisited = HashSet::with_capacity(path.len());
		let mut last = Point(0, 0);
		for segment in path {
			for _ in 0..segment.distance {
				last = last.translate(segment.direction, 1);
				tour.insert(last.clone());
			}
		}
		tours.push(tour);
	}
	tours[0]
		.intersection(&tours[1])
		.map(Point::manhattan_distance_from_origin)
		.min()
		.unwrap()
}

#[aoc(day3, part2)]
fn s2(paths: &[Path]) -> u32 {
	// It would have actually been much easier to use a HashMap instead of a
	// HashSet here. With this method, I had to kind of bend over backwards to
	// implement PartialEq, Eq, and Hash on Counted. Definitely, a simple HashMap
	// with Point as the key and u32 as the value would have been nicer.
	let mut tours: Vec<HashSet<Counted<Point>>> = Vec::with_capacity(2);
	for path in paths {
		let mut tour: HashSet<Counted<Point>> =
			HashSet::with_capacity(path.len());
		let mut last = Counted {
			x: Point(0, 0),
			count: 0,
		};
		for segment in path {
			for _ in 0..segment.distance {
				last = Counted {
					x: last.x.translate(segment.direction, 1),
					count: last.count + 1,
				};
				if !tour.contains(&last) {
					tour.insert(last.clone());
				}
			}
		}
		tours.push(tour);
	}
	tours[0]
		.intersection(&tours[1])
		.map(|x| {
			tours[0].get(x).unwrap().count + tours[1].get(x).unwrap().count
		})
		.min()
		.unwrap()
}
