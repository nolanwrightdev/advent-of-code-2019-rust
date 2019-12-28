use aoc_runner_derive::{aoc, aoc_generator};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

struct Node<'a> {
	id: String,
	satellites: RefCell<Vec<&'a Node<'a>>>,
}

impl<'a> Node<'a> {
	fn new(id: String) -> Node<'a> {
		Node {
			id,
			satellites: RefCell::new(Vec::new()),
		}
	}
	fn add_satellite(&self, satellite: &'a Node<'a>) {
		self.satellites.borrow_mut().push(satellite);
	}
}

#[aoc_generator(day6)]
fn g(input: &str) -> HashMap<String, Vec<String>> {
	let mut map: HashMap<String, Vec<String>> = HashMap::new();
	input.lines().for_each(|line| {
		let mut parts = line.split(')');
		let first = parts.next().unwrap();
		let second = parts.next().unwrap().to_owned();
		match map.get_mut(first) {
			Some(vec) => {
				vec.push(second);
			}
			None => {
				let mut vec = Vec::new();
				vec.push(second);
				map.insert(first.to_owned(), vec);
			}
		}
	});
	map
}

#[aoc(day6, part1)]
fn s1(m: &HashMap<String, Vec<String>>) -> u32 {
	let mut map: HashMap<String, Node> = HashMap::new();
	// map identifiers to node structs
	for (k, v) in m {
		if !map.contains_key(k) {
			map.insert(k.clone(), Node::new(k.clone()));
		}
		for v in v {
			if !map.contains_key(v) {
				map.insert(v.clone(), Node::new(v.clone()));
			}
		}
	}
	// assemble tree of nodes
	for (k, v) in m {
		for v in v {
			let satellite = map.get(v).unwrap();
			let node = map.get(k).unwrap();
			node.add_satellite(satellite);
		}
	}
	// count up the depths
	let root = map.get("COM").unwrap();
	let mut count = 0u32;
	let mut queue: Vec<(&Node, u32)> = vec![(root, 0)];
	while !queue.is_empty() {
		let mut new_queue: Vec<(&Node, u32)> = Vec::new();
		for (node, depth) in queue.iter() {
			count += depth;
			for satellite in node.satellites.borrow().iter() {
				new_queue.push((satellite, depth + 1));
			}
		}
		queue = new_queue;
	}
	count
}

#[aoc(day6, part2)]
fn s2(m: &HashMap<String, Vec<String>>) -> u32 {
	let mut map: HashMap<String, (Node, Option<&str>)> = HashMap::new();
	// map identifiers to node structs
	for (k, v) in m {
		if !map.contains_key(k) {
			map.insert(k.clone(), (Node::new(k.clone()), None));
		}
		for v in v {
			match map.get(v) {
				Some((_, None)) => {
					let (node, _) = map.remove(v).unwrap();
					map.insert(v.to_owned(), (node, Some(k)));
				}
				None => {
					map.insert(v.clone(), (Node::new(v.clone()), Some(k)));
				}
				_ => panic!("node having multiple parents not expected"),
			}
		}
	}
	// assemble tree of nodes
	for (k, v) in m {
		for v in v {
			let (satellite, _) = map.get(v).unwrap();
			let (node, _) = map.get(k).unwrap();
			node.add_satellite(satellite);
		}
	}
	// find least distance
	let mut visited: HashSet<&str> = HashSet::new();
	visited.insert("YOU");
	let (_, maybe_you_parent_id) = map.get("YOU").unwrap();
	let you_parent_id = maybe_you_parent_id.unwrap();
	let mut queue: Vec<(&str, u32)> = vec![(you_parent_id, 0)];
	while let Some((id, distance)) = queue.pop() {
		if id == "SAN" {
			return distance - 1;
		}
		let (node, maybe_parent_id) = map.get(id).unwrap();
		if let Some(parent_id) = maybe_parent_id {
			if !visited.contains(parent_id) {
				queue.push((parent_id, distance + 1));
			}
		}
		for satellite in node.satellites.borrow().iter() {
			if !visited.contains(&satellite.id.as_ref()) {
				queue.push((&satellite.id, distance + 1));
			}
		}
		visited.insert(id);
	}
	panic!("never found santa?!")
}
