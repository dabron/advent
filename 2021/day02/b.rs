use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut aim = 0;
    let mut depth = 0;
    let mut pos = 0;

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        let split: Vec<&str> = input.split(' ').collect();
        if split.iter().count() >= 2 {
            let dir = split[0];
            match split[1].parse::<u32>() {
                Ok(x) => {
                    match dir {
                        "up" => aim -= x,
                        "down" => aim += x,
                        "forward" => {
                            pos += x;
                            depth += aim * x;
                        },
                        _ => {},
                    }
                }
                Err(..) => {},
            }
        }
    }

    println!("location ({}, {}): {}", depth, pos, depth * pos);
}
