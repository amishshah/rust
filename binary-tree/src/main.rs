// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370

use std::io::prelude::*;
use std::io;

struct Node {
	value: String,
	left: Option<Box<Node<>>>,
	right: Option<Box<Node<>>>,
}

impl Node {
	fn insert(&mut self, new_value: String) {
		if new_value == self.value {
			println!("Discarding \"{}\" as it already exists", new_value);
			return
		}
		let (direction, target_node) = if new_value > self.value {
			("right", &mut self.right)
		} else {
			("left", &mut self.left)
		};
		match target_node {
			// https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html
			Some(ref mut node) => {
				println!("Going to the {} subnode of \"{}\"", direction, self.value);
				node.insert(new_value);
			},
			None => {
				println!("Inserting \"{}\" to the {} subnode of \"{}\"", new_value, direction, self.value);
				let new_node = Node { value: new_value, left: None, right: None };
				let boxed_node = Box::new(new_node);
				*target_node = Some(boxed_node);
			}
		}
	}
}

fn get_input() -> String {
	print!("\nEnter a string (or .quit): ");
	io::stdout().flush().expect("Error");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input!");
	input
}

fn main() {
	let mut tree = Node { value: String::from("middle"), left: None, right: None };
	print!("Created root \"{}\"", tree.value);
	loop {
		let input = get_input().trim().to_owned();
		if input == ".quit" {
			break;
		} else {
			tree.insert(input);
		}
	}
}
