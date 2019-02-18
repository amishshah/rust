use std::io;

fn get_string() -> Result<String, &'static str> {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Ok(_) => Ok(input),
		Err(_) => Err("Couldn't read input")
	}
}

fn input_numbers() -> Vec<isize> {
	let mut numbers: Vec<isize> = Vec::new();
	loop {
		println!("Enter an integer or type \"done\" to sort ({} items currently):", numbers.len());
		let input = get_string().unwrap();
		if input.trim() == "done" {
			break
		} else {
			numbers.push(match input.trim().parse() {
				Ok(n) => n,
				_ => continue
			});
		}
	}
	numbers
}

fn main() {
	let mut numbers = input_numbers();
	let mut iterations = 0;
	loop {
		let mut changes = 0;
		for i in 0..(numbers.len()-1) {
			let (num1, num2) = (numbers[i], numbers[i+1]);
			if num1 > num2 {
				numbers[i] = num2;
				numbers[i+1] = num1;
				changes += 1;
			}
		}
		iterations += 1;
		if changes == 0 {
			break
		}
	}
	println!("Sorted list ({} iterations): {:?}", iterations, numbers);
}
