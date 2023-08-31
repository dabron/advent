use std::io::{self, BufRead};

fn bin_to_dec(bin: &String) -> u32 {
    let mut dec = 0;
    let chars: Vec<char> = bin.chars().collect();
    let size = chars.len() as u32;
    for i in 0..size {
        if chars[i as usize] == '1' {
            dec += u32::pow(2, size - i - 1);
        }
    }
    dec
}

fn should_remove(s: &String, i: usize, c: char) -> bool {
    let chars: Vec<char> = s.chars().collect();
    chars[i] == c
}

fn remove(v: &mut Vec<String>, i: usize, c: char) {
    let mut index = 0;
    while index < v.len() {
        if should_remove(&v[index], i, c) {
            v.remove(index);
        }
        else {
            index += 1;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut o2_vec: Vec<String> = Vec::new();
    let mut co2_vec: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        let input = line.unwrap();
        o2_vec.push(input.clone());
        co2_vec.push(input);
    }

    println!("count: {} {}", o2_vec.iter().count(), co2_vec.iter().count());

    let mut i = 0;
    while o2_vec.iter().count() > 1 {
        let mut count = (0, 0);
        for s in o2_vec.iter() {
            let chars: Vec<char> = s.chars().collect();
            if chars[i] == '1' {
                count.1 += 1;
            }
            else {
                count.0 += 1;
            }
        }
        if count.1 >= count.0 {
            remove(&mut o2_vec, i, '0');
        }
        else {
            remove(&mut o2_vec, i, '1');
        }
        i += 1;
    }

    i = 0;
    while co2_vec.iter().count() > 1 {
        let mut count = (0, 0);
        for s in co2_vec.iter() {
            let chars: Vec<char> = s.chars().collect();
            if chars[i] == '1' {
                count.1 += 1;
            }
            else {
                count.0 += 1;
            }
        }
        if count.1 >= count.0 {
            remove(&mut co2_vec, i, '1');
        }
        else {
            remove(&mut co2_vec, i, '0');
        }
        i += 1;
    }
    println!("count: {} {}", o2_vec.iter().count(), co2_vec.iter().count());
    println!("binary: {} {}", o2_vec[0], co2_vec[0]);

    let o2 = bin_to_dec(&o2_vec[0]);
    let co2 = bin_to_dec(&co2_vec[0]);
    println!("o2={}, co2={}: {}", o2, co2, o2 * co2);
}