use std::io::{self, BufRead};

fn get_item_priority(item: char) -> u32 {
	if item >= 'a' && item <= 'z' {
		return (item as u32) - ('a' as u32) + 1;
	} else if item >= 'A' && item <= 'Z' {
		return (item as u32) - ('A' as u32) + 27;
	}
	0
}

fn get_priority(bag1: &String, bag2: &String, bag3: &String) -> u32 {
	for x in bag1.chars().collect::<Vec<char>>() {
		for y in bag2.chars().collect::<Vec<char>>() {
			if x == y {
				for z in bag3.chars().collect::<Vec<char>>() {
					if x == z {
						return get_item_priority(x);
					}
				}
			}
		}
	}
	0
}

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut bag1: String = "".to_string();
	let mut bag2: String = "".to_string();
	let mut bag3: String;
	let mut bag_count = 0;
	let mut priority = 0;

	while let Some(line) = lines.next() {
		bag3 = bag2;
		bag2 = bag1;
		bag1 = line.unwrap();
		bag_count += 1;

		if bag_count == 3 {
			priority += get_priority(&bag1, &bag2, &bag3);
			bag_count = 0;
		}
	}

	println!("priority: {}", priority);
}