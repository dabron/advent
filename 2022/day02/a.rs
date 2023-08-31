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

		match mine {
			'X' => score = 1,
			'Y' => score = 2,
			'Z' => score = 3,
			_ => {}
		}

		match theirs {
			'A' => {
				match mine {
					'X' => score += 3,
					'Y' => score += 6,
					_ => {}
				}
			},
			'B' => {
				match mine {
					'Y' => score += 3,
					'Z' => score += 6,
					_ => {}
				}
			},
			'C' => {
				match mine {
					'Z' => score += 3,
					'X' => score += 6,
					_ => {}
				}
			},
			_ => {}
		}

		total += score;
	}

	println!("score: {}", total);
}