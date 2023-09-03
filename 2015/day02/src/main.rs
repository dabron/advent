extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

fn calculate_ribbon_needed(l: u32, w: u32, h: u32) -> u32 {
    let x = if l >= w && l >= h {w + h} else if w >= l && w >= h {l + h} else {l + w};
    x * 2 + l * w * h
}

fn calculate_paper_needed(l: u32, w: u32, h: u32) -> u32 {
    let a = l * w;
    let b = l * h;
    let c = w * h;
    let slack = if a < b {if a < c {a} else {c}} else if b < c {b} else {c};
    a * 2 + b * 2 + c * 2 + slack
}

fn parse_dimensions(input: &str) -> (u32, u32, u32) {
    let re = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut l = 0;
    let mut w = 0;
    let mut h = 0;
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<u32>() {Ok(x) => {l = x;}, Err(..) => {}}
        match cap[2].parse::<u32>() {Ok(x) => {w = x;}, Err(..) => {}}
        match cap[3].parse::<u32>() {Ok(x) => {h = x;}, Err(..) => {}}
    }
    (l, w, h)
}

fn main() {
    let mut input = "2x3x4";
    let (l, w, h) = parse_dimensions(input);
    println!("needed paper ({}): {} sq ft", input, calculate_paper_needed(l, w, h));
    println!("needed ribbon ({}): {} ft", input, calculate_ribbon_needed(l, w, h));

    input = "1x1x10";
    let (l, w, h) = parse_dimensions(input);
    println!("needed paper ({}): {} sq ft", input, calculate_paper_needed(l, w, h));
    println!("needed ribbon ({}): {} ft", input, calculate_ribbon_needed(l, w, h));

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total_needed_paper = 0;
    let mut total_needed_ribbon = 0;
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        let (l, w, h) = parse_dimensions(&input);
        total_needed_paper += calculate_paper_needed(l, w, h);
        total_needed_ribbon += calculate_ribbon_needed(l, w, h);
    }
    println!("total needed paper: {} sq ft", total_needed_paper);
    println!("total needed ribbon: {} ft", total_needed_ribbon);
}
