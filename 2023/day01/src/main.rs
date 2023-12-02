use std::io::{self, BufRead};

fn get_first_digit(input: &str) -> u32 {
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap();
        }
    }
    println!("failed to find first digit in {}", input);
    0
}

fn get_last_digit(input: &str) -> u32 {
    for c in input.chars().rev() {
        if c >= '0' && c <= '9' {
            return c.to_digit(10).unwrap();
        }
    }
    println!("failed to find last digit in {}", input);
    0
}

fn get_first_digit2(input: &str) -> u32 {
    let mut lowest = usize::MAX;
    let mut value = 0u32;
    let index = input.find("one");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 1;
        }
    }
    let index = input.find("two");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 2;
        }
    }
    let index = input.find("three");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 3;
        }
    }
    let index = input.find("four");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 4;
        }
    }
    let index = input.find("five");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 5;
        }
    }
    let index = input.find("six");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 6;
        }
    }
    let index = input.find("seven");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 7;
        }
    }
    let index = input.find("eight");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 8;
        }
    }
    let index = input.find("nine");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 9;
        }
    }
    let index = input.find("0");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 0;
        }
    }
    let index = input.find("1");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 1;
        }
    }
    let index = input.find("2");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 2;
        }
    }
    let index = input.find("3");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 3;
        }
    }
    let index = input.find("4");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 4;
        }
    }
    let index = input.find("5");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 5;
        }
    }
    let index = input.find("6");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 6;
        }
    }
    let index = input.find("7");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 7;
        }
    }
    let index = input.find("8");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            lowest = i;
            value = 8;
        }
    }
    let index = input.find("9");
    if index != None {
        let i = index.unwrap();
        if i < lowest {
            value = 9;
        }
    }
    value
}

fn get_last_digit2(input: &str) -> u32 {
    let mut highest: usize = 0;
    let mut value = 0u32;
    let index = input.rfind("one");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 1;
        }
    }
    let index = input.rfind("two");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 2;
        }
    }
    let index = input.rfind("three");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 3;
        }
    }
    let index = input.rfind("four");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 4;
        }
    }
    let index = input.rfind("five");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 5;
        }
    }
    let index = input.rfind("six");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 6;
        }
    }
    let index = input.rfind("seven");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 7;
        }
    }
    let index = input.rfind("eight");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 8;
        }
    }
    let index = input.rfind("nine");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 9;
        }
    }
    let index = input.rfind("0");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 0;
        }
    }
    let index = input.rfind("1");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 1;
        }
    }
    let index = input.rfind("2");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 2;
        }
    }
    let index = input.rfind("3");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 3;
        }
    }
    let index = input.rfind("4");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 4;
        }
    }
    let index = input.rfind("5");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 5;
        }
    }
    let index = input.rfind("6");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 6;
        }
    }
    let index = input.rfind("7");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 7;
        }
    }
    let index = input.rfind("8");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            highest = i;
            value = 8;
        }
    }
    let index = input.rfind("9");
    if index != None {
        let i = index.unwrap();
        if i >= highest {
            value = 9;
        }
    }
    value
}

fn get_digit(input: &str) -> u32 {
    let first = get_first_digit(&input);
    let last = get_last_digit(&input);
    first * 10 + last
}

fn get_digit2(input: &str) -> u32 {
    let first = get_first_digit2(&input);
    let last = get_last_digit2(&input);
    first * 10 + last
}

fn print_digits(input: &str) -> u32 {
    let digit = get_digit(input);
    println!("{} -> {}", input, digit);
    digit
}

fn print_digits2(input: &str) -> u32 {
    let digit = get_digit2(input);
    println!("{} -> {}", input, digit);
    digit
}

fn main() {
    let a = print_digits("1abc2");
    let b = print_digits("pqr3stu8vwx");
    let c = print_digits("a1b2c3d4e5f");
    let d = print_digits("treb7uchet");
    println!("total: {}", a + b + c + d);

    let e = print_digits2("two1nine");
    let f = print_digits2("eightwothree");
    let g = print_digits2("abcone2threexyz");
    let h = print_digits2("xtwone3four");
    let i = print_digits2("4nineeightseven2");
    let j = print_digits2("zoneight234");
    let k = print_digits2("7pqrstsixteen");
    println!("total 2: {}", e + f + g + h + i + j + k);

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut sum = 0u32;
    let mut sum2 = 0u32;
    while let Some(line) = lines.next() {
        let input = line.unwrap();
        sum += get_digit(&input);
        sum2 += get_digit(&input);
    }
    println!("sum: {}", sum);
    println!("sum 2: {}", sum2);
}
