use std::io;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Player {
	A,
	B
}

impl fmt::Display for Player {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			Player::A => "X",
			Player::B => "O"
		})
	}
}

#[derive(Copy, Clone, PartialEq)]
enum CellType {
	Empty,
	Claimed(Player),
}

enum State {
	Playing,
	NoWinner,
	Winner(Player),
}

impl fmt::Display for CellType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match *self {
			CellType::Empty => String::from("-"),
			CellType::Claimed(player) => format!("{}", player)
		})
	}
}

struct Grid {
	cells: [CellType; 9],
}

impl Grid {
	fn outcomes(&self) -> [(CellType, CellType, CellType); 8] {
		[
			(self.cells[0], self.cells[1], self.cells[2]),
			(self.cells[3], self.cells[4], self.cells[5]),
			(self.cells[6], self.cells[7], self.cells[8]),
			(self.cells[0], self.cells[3], self.cells[6]),
			(self.cells[1], self.cells[4], self.cells[7]),
			(self.cells[2], self.cells[5], self.cells[8]),
			(self.cells[0], self.cells[4], self.cells[8]),
			(self.cells[2], self.cells[4], self.cells[6]),
		]
	}

	fn calculate_state(&self) -> State {
		let outcomes = self.outcomes();
		for outcome in outcomes.iter() {
			if let (CellType::Claimed(a), CellType::Claimed(b), CellType::Claimed(c)) = outcome {
				if a == b && a == c {
					return State::Winner(a.clone());
				}
			}
		}
		let empty_cells = self.cells.iter().filter(|cell| **cell == CellType::Empty).count();
		match empty_cells {
			0 => State::NoWinner,
			_ => State::Playing
		}
	}

	fn claim(&mut self, place: usize, player: Player) -> Result<(), &'static str> {
		match self.cells[place] {
			CellType::Empty => Ok(self.cells[place] = CellType::Claimed(player)),
			CellType::Claimed(_) => Err("Cell is claimed!"),
		}
	}
}

impl fmt::Display for Grid {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}{}\n{}{}{}\n{}{}{}", self.cells[0], self.cells[1], self.cells[2],
			self.cells[3], self.cells[4], self.cells[5],
			self.cells[6], self.cells[7], self.cells[8])
	}
}

fn read_input() -> Result<String, &'static str> {
	let mut input = String::new();
	match io::stdin().read_line(&mut input) {
		Err(_) => Err("Failed to read string"),
		Ok(_) => Ok(input)
	}
}

fn read_int() -> Result<usize, &'static str> {
	let input = read_input()?;
	match input.trim().parse() {
		Ok(n) => Ok(n),
		Err(_) => Err("Failed to parse as usize")
	}
}

fn print_grid(grid: &Grid) {
	println!("\n\nGrid:\n{}", grid);
}

fn main() {
	let mut grid = Grid { cells: [CellType::Empty; 9] };

	let mut player = Player::A;

	loop {
		print_grid(&grid);
		println!("{}'s turn (enter a position from 0 to 8):", player);
		if let Ok(index) = read_int() {
			if index > 8 {
				continue
			}
			if let Ok(_) = grid.claim(index, player) {
				match grid.calculate_state() {
					State::Winner(player) => {
						print_grid(&grid);
						println!("{} wins!", player);
						break;
					},
					State::NoWinner => {
						print_grid(&grid);
						println!("Game over! Nobody won!");
						break;
					}
					State::Playing => {
						player = match player {
							Player::A => Player::B,
							Player::B => Player::A
						}
					}
				}
			}
		}
	}
}
