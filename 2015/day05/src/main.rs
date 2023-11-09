use std::io::{self, BufRead};

fn has_three_vowels(input: &str) -> bool {
    let mut vowel_count = 0u32;
    for c in input.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
            if vowel_count >= 3 {
                return true;
            }
        }
    }
    false
}

fn has_double_letter(input: &str) -> bool {
    let mut d = '\0';
    for c in input.chars() {
        if c == d {
            return true;
        }
        d = c;
    }
    false
}

fn has_no_forbidden_pairs(input: &str) -> bool {
    if input.contains("ab") {
        return false;
    }
    if input.contains("cd") {
        return false;
    }
    if input.contains("pq") {
        return false;
    }
    if input.contains("xy") {
        return false;
    }
    true
}

fn is_nice(input: &str) -> bool {
    has_three_vowels(input) &&
    has_double_letter(input) &&
    has_no_forbidden_pairs(input)
}

fn has_pair_twice(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    let len = chars.len();
    for i in 0..(len - 1) {
        let c1 = chars[i];
        let c2 = chars[i + 1];
        for j in (i + 2)..(len - 1) {
            let d1 = chars[j];
            let d2 = chars[j + 1];
            if c1 == d1 && c2 == d2 {
                return true;
            }
        }
    }
    false
}

fn has_separated_duplicate(input: &str) -> bool {
    let chars: Vec<_> = input.chars().collect();
    let len = chars.len();
    for i in 0..(len - 2) {
        let c = chars[i];
        let d = chars[i + 2];
        if c == d {
            return true;
        }
    }
    false
}

fn is_nice2(input: &str) -> bool {
    has_pair_twice(input) &&
    has_separated_duplicate(input)
}

fn print_moral_status(input: &str) {
    let status = if is_nice(input) {"nice"} else {"naughty"};
    let status2 = if is_nice2(input) {"nice"} else {"naughty"};
    println!("{} is {} and {}", input, status, status2);
}


fn main() {
    print_moral_status("ugknbfddgicrmopn");
    print_moral_status("aaa");
    print_moral_status("jchzalrnumimnmhp");
    print_moral_status("haegwjzuvuyypxyu");
    print_moral_status("dvszwmarrgswjxmb");

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut nice_count = 0u32;
    let mut nice_count2 = 0u32;
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        print_moral_status(input.as_str());
        if is_nice(input.as_str()) {
            nice_count += 1;
        }
        if is_nice2(input.as_str()) {
            nice_count2 += 1;
        }
    }
    println!("nice count 1: {}", nice_count);
    println!("nice count 2: {}", nice_count2);
}
