use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let Some(line) = lines.next();
    let draw = line.unwrap().split(',');

    for d in draw {
        println!("draw: {}", d);
    }
}
