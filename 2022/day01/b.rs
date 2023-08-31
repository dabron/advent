use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut total = 0u32;
	let mut top1 = 0u32;
	let mut top2 = 0u32;
	let mut top3 = 0u32;

	while let Some(line) = lines.next() {
		let input = line.unwrap();
		match input.parse::<u32>() {
			Ok(value) => {
				total += value;
			},
			Err(..) => {
				if total >= top1 {
					top3 = top2;
					top2 = top1;
					top1 = total;
				} else if total >= top2 {
					top3 = top2;
					top2 = total;
				} else if total >= top3 {
					top3 = total;
				}
				total = 0;
			}
		}
	}

	println!("top 3: {} {} {}", top1, top2, top3);
	println!("total: {}", top1 + top2 + top3);
}
