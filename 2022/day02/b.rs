use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut total = 0u32;

	while let Some(line) = lines.next() {
		let input = line.unwrap();
		let theirs = input.chars().nth(0).unwrap();
		let mine = input.chars().nth(2).unwrap();
		let mut score = 0u32;

		match theirs {
			'A' => {
				match mine {
					'X' => score = 3 + 0,
					'Y' => score = 1 + 3,
					'Z' => score = 2 + 6,
					_ => {}
				}
			},
			'B' => {
				match mine {
					'X' => score = 1 + 0,
					'Y' => score = 2 + 3,
					'Z' => score = 3 + 6,
					_ => {}
				}
			},
			'C' => {
				match mine {
					'X' => score = 2 + 0,
					'Y' => score = 3 + 3,
					'Z' => score = 1 + 6,
					_ => {}
				}
			},
			_ => {}
		}

		total += score;
	}

	println!("score: {}", total);
}