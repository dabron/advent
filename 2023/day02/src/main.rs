extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

fn parse_game(input: &str) -> (u32, u32, u32, u32) {
    let mut id = 0u32;
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;

    let re = Regex::new(r"Game (\d+):").unwrap();
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<u32>() {Ok(x) => {id = x;}, Err(..) => {}}
    }

    let re = Regex::new(r"(\d+) red").unwrap();
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<u32>() {Ok(x) => {if x > r {r = x;}}, Err(..) => {}}
    }

    let re = Regex::new(r"(\d+) green").unwrap();
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<u32>() {Ok(x) => {if x > g {g = x;}}, Err(..) => {}}
    }

    let re = Regex::new(r"(\d+) blue").unwrap();
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<u32>() {Ok(x) => {if x > b {b = x;}}, Err(..) => {}}
    }

    (id, r, g, b)
}

fn is_valid(r: u32, g: u32, b: u32, max_r: u32, max_g: u32, max_b: u32) -> bool {
    if r > max_r || g > max_g || b > max_b {
        return false;
    }
    true
}

fn print_game(input: &str) -> (u32, u32, u32, u32) {
    let (id, r, g, b) = parse_game(input);
    println!("Game {}: {} red, {} green, {} blue, power: {}", id, r, g, b, r * g * b);
    (id, r, g, b)
}

fn main() {
    let mut valid = 0u32;
    let mut power = 0u32;
    let max_r = 12u32;
    let max_g = 13u32;
    let max_b = 14u32;

    let (id, r, g, b) = print_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
    power += r * g * b;
    let (id, r, g, b) = print_game("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
    if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
    power += r * g * b;
    let (id, r, g, b) = print_game("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
    power += r * g * b;
    let (id, r, g, b) = print_game("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
    if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
    power += r * g * b;
    let (id, r, g, b) = print_game("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
    power += r * g * b;
    println!("valid: {}, power: {}", valid, power);
    
    valid = 0;
    power = 0;
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        let (id, r, g, b) = parse_game(&input);
        if is_valid(r, g, b, max_r, max_g, max_b) {valid += id;}
        power += r * g * b;
    }
    println!("valid: {}, power: {}", valid, power);
}
