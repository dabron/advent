// extern crate regex;

use std::io::{self, BufRead};
// use regex::Regex;

fn setup_sample_crates(stacks: &mut Vec<Vec<char>>) {
	stacks.push(['Z', 'N'].to_vec());
	stacks.push(['M', 'C', 'D'].to_vec());
	stacks.push(['P'].to_vec());
}

fn move_crates(_stacks: &mut [Vec<char>], _count: u32, _from: u32, _to: u32) {

}

fn print_crates(stacks: &[Vec<char>]) {
	for i in 0..stacks.len() {
		print!("{}: ", i + 1);
		for j in 0..stacks[i].len() {
			print!("{} ", stacks[i][j]);
		}
		println!("");
	}
}

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut stacks = Vec::new();
	// let mut count = 0;
	// let mut from = 0;
	// let mut to = 0;

	// for i in 0..stacks.len() {
	// 	stacks[i] = Vec::<char>::new();
	// }
	setup_sample_crates(&mut stacks);

	while let Some(line) = lines.next() {
		let _input = line.unwrap();
		// let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$");
		// for cap in re.captures_iter(&input) {
		// 	match cap[1].parse::<u32>() {Ok(x) => {count = x;}, Err(..) => {}}
		// 	match cap[1].parse::<u32>() {Ok(x) => {from  = x;}, Err(..) => {}}
		// 	match cap[1].parse::<u32>() {Ok(x) => {to    = x;}, Err(..) => {}}
		// 	break;
		// }

		// move_crates(&mut stacks, count, from, to);
	}

	print_crates(&stacks);
}