use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut gamma = 0;
    let mut epsilon = 0;
    let mut ones: Vec<u32> = Vec::new();
    let mut zeros: Vec<u32> = Vec::new();
    let mut size: usize = 0;

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        let bits: Vec<char> = input.chars().collect();

        if bits.iter().count() > size {
            for _ in size .. bits.iter().count() {
                ones.push(0);
                zeros.push(0);
            }
            size = bits.iter().count();
        }

        for i in 0 .. size {
            if bits[i] == '1' {
                ones[i] += 1;
            }
            else {
                zeros[i] += 1;
            }
        }
    }

    for i in 0 .. size {
        let exp = (size - i - 1) as u32;
        if ones[i] > zeros[i] {
            gamma += u32::pow(2, exp);
        }
        else {
            epsilon += u32::pow(2, exp);
        }
    }

    println!("gamma={}, epsilon={}: {}", gamma, epsilon, gamma * epsilon);
}
