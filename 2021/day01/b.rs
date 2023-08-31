use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut larger = 0u32;
    let mut w = 0u32;
    let mut x = 0u32;
    let mut y = 0u32;

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        match input.parse::<u32>() {
            Ok(z) => {
                if (w != 0) && ((x + y + z) > (w + x + y)) {
                    larger += 1;
                }
                w = x;
                x = y;
                y = z;
            },
            Err(..) => {},
        }
    }

    println!("larger count: {}", larger);
}
