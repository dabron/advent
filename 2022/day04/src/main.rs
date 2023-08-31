extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

// fn is_full_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
// 	if (start1 == start2) ||
// 	   (end1 == end2) ||
// 	   (start1 > start2 && end1 < end2) ||
// 	   (start2 > start1 && end2 < end1) {
// 		return true;
// 	}

// 	false
// }

fn is_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
	if (end1 < start2) ||
	   (end2 < start1) {
		return false;
	}

	true
}

fn main() {
	let stdin = io::stdin();
	let mut lines = stdin.lock().lines();
	let mut start1 = 0;
	let mut end1 = 0;
	let mut start2 = 0;
	let mut end2 = 0;
	let mut overlap_count = 0;

	while let Some(line) = lines.next() {
		let input = line.unwrap();
		let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
		for cap in re.captures_iter(&input) {
			match cap[1].parse::<u32>() {Ok(x) => {start1 = x;}, Err(..) => {}}
			match cap[2].parse::<u32>() {Ok(x) => {end1   = x;}, Err(..) => {}}
			match cap[3].parse::<u32>() {Ok(x) => {start2 = x;}, Err(..) => {}}
			match cap[4].parse::<u32>() {Ok(x) => {end2   = x;}, Err(..) => {}}
			break;
		}
		if is_overlap(start1, end1, start2, end2) {
			overlap_count += 1;
		}
	}

	println!("Overlapped: {}", overlap_count);
}