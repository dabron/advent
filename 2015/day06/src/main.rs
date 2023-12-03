extern crate regex;

use std::io::{self, BufRead};
use regex::Regex;

const SIZE: usize = 1000;

fn turn_on(grid: &mut [[bool; SIZE]; SIZE], grid2: &mut [[u32; SIZE]; SIZE], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = true;
            grid2[x][y] += 1;
        }
    }
}

fn turn_off(grid: &mut [[bool; SIZE]; SIZE], grid2: &mut [[u32; SIZE]; SIZE], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = false;
            if grid2[x][y] > 0 {
                grid2[x][y] -= 1;
            }
        }
    }
}

fn toggle(grid: &mut [[bool; SIZE]; SIZE], grid2: &mut [[u32; SIZE]; SIZE], x1: usize, y1: usize, x2: usize, y2: usize) {
    for x in x1..=x2 {
        for y in y1..=y2 {
            grid[x][y] = !grid[x][y];
            grid2[x][y] += 2;
        }
    }
}

fn count(grid: &mut [[bool; SIZE]; SIZE], grid2: &mut [[u32; SIZE]; SIZE]) -> (u32, u32) {
    let mut count = 0u32;
    let mut count2 = 0u32;
    for x in 0..SIZE {
        for y in 0..SIZE {
            if grid[x][y] {
                count += 1;
            }
            count2 += grid2[x][y];
        }
    }
    (count, count2)
}

fn parse(input: &str) -> (String, usize, usize, usize, usize) {
    let re = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut command = String::new();
    let mut x1: usize = 0;
    let mut y1: usize = 0;
    let mut x2: usize = 0;
    let mut y2: usize = 0;
    for cap in re.captures_iter(&input) {
        match cap[1].parse::<String>() {Ok(x) => {command = x;}, Err(..) => {println!("parse error: command");}}
        match cap[2].parse::<usize>() {Ok(x) => {x1 = x;}, Err(..) => {println!("parse error: x1");}}
        match cap[3].parse::<usize>() {Ok(x) => {y1 = x;}, Err(..) => {println!("parse error: y1");}}
        match cap[4].parse::<usize>() {Ok(x) => {x2 = x;}, Err(..) => {println!("parse error: x2");}}
        match cap[5].parse::<usize>() {Ok(x) => {y2 = x;}, Err(..) => {println!("parse error: y2");}}
    }
    (command, x1, y1, x2, y2)
}

fn main() {
    let mut grid: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];
    let mut grid2: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    
    turn_on(&mut grid, &mut grid2, 0, 0, 999, 999);
    toggle(&mut grid, &mut grid2, 0, 0, 999, 0);
    turn_off(&mut grid, &mut grid2, 499, 499, 500, 500);
    let (count1, count2) = count(&mut grid, &mut grid2);
    println!("count1: {}, count2: {}", count1, count2);
    
    turn_off(&mut grid, &mut grid2, 0, 0, SIZE - 1, SIZE - 1);
    grid2 = [[0; SIZE]; SIZE];

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        let (command, x1, y1, x2, y2) = parse(&input);
        match command.as_str() {
            "turn on" => {turn_on(&mut grid, &mut grid2, x1, y1, x2, y2);}
            "turn off" => {turn_off(&mut grid, &mut grid2, x1, y1, x2, y2);}
            "toggle" => {toggle(&mut grid, &mut grid2, x1, y1, x2, y2);}
            _ => {println!("bad command: {}", command);}
        }
    }
    let (count1, count2) = count(&mut grid, &mut grid2);
    println!("count1: {}, count2: {}", count1, count2);
}
