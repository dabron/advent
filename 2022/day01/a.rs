use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut total = 0u32;
    let mut max = 0u32;

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        match input.parse::<u32>() {
            Ok(value) => {
                total += value
            },
            Err(..) => {
                if total > max {
                    max = total
                }
                total = 0
            }
        }
    }

    println!("max: {}", max);
}
