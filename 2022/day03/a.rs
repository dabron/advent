use std::io::{self, BufRead};

fn get_priority(input: &String) -> u32 {
	let chars: Vec<char> = input.chars().collect();
	let half = input.len() / 2;
	let full = input.len();
	let mut found: bool = false;
	let mut item: char = '\0';
	let mut priority: u32 = 0;

	for i in 0..half {
		for j in half..full {
			if chars[i] == chars[j] {
				found = true;
				break;
			}
		}
		if found {
			item = chars[i];
			break;
		}
	}

	if item >= 'a' && item <= 'z' {
		priority = (item as u32) - ('a' as u32) + 1;
	} else if item >= 'A' && item <= 'Z' {
		priority = (item as u32) - ('A' as u32) + 27;
	}

	priority
}

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut priority = 0;

	while let Some(line) = lines.next() {
		let input = line.unwrap();
		priority += get_priority(&input);
	}

	println!("priority: {}", priority);
}