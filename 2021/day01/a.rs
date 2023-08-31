use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut larger = 0u32;
    let mut prev = u32::MAX;

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        match input.parse::<u32>() {
            Ok(cur) => {
                if cur > prev {
                    larger += 1;
                }
                prev = cur;
            },
            Err(..) => {},
        }
    }

    println!("larger count: {}", larger);
}
