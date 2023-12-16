use std::fs;

const MAX_INSTANCES: usize = 10;

fn parse_card(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut colon: usize = 0;
    let mut pipe: usize = 0;
    let mut i: usize = 0;
    for c in input.chars() {
        match c {
            ':' => {colon = i;},
            '|' => {pipe = i;},
            _ => {}
        }
        i += 1;
    }
    let mut wins: Vec<u32> = Vec::new();
    for x in input[colon+1..pipe].split(" ") {
        if x == "" {continue;}
        wins.push(x.parse().unwrap());
    }
    let mut nums: Vec<u32> = Vec::new();
    for x in input[pipe+1..i].split(" ") {
        if x == "" {continue;}
        nums.push(x.parse().unwrap());
    }
    (wins, nums)
}

fn calc_card(input: &str) -> u32 {
    let mut points = 0u32;
    let (wins, nums) = parse_card(input);
    for n in nums {
        if wins.contains(&n) {
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }
    }
    points
}

fn calc_card2(input: &str, instances: &mut[u64; MAX_INSTANCES]) -> u64 {
    let (wins, nums) = parse_card(input);
    let mut matches : usize = 0;
    for n in nums {
        if wins.contains(&n) {
            matches += 1;
        }
    }
    let num = get_and_shift(instances);
    for i in 0..matches {
        instances[i] += num;
    }
    println!("num={}", num);
    num
}

fn get_and_shift(instances: &mut[u64; MAX_INSTANCES]) -> u64 {
    let num = instances[0];
    for i in 0..MAX_INSTANCES-1 {
        instances[i] = instances[i+1];
    }
    instances[MAX_INSTANCES-1] = 1;
    num
}

fn main() {
    let mut instances : [u64; MAX_INSTANCES] = [1; MAX_INSTANCES];

    let mut sample1_sum = 0u32;
    sample1_sum += calc_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
    sample1_sum += calc_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
    sample1_sum += calc_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
    sample1_sum += calc_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
    sample1_sum += calc_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
    sample1_sum += calc_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
    println!("sample1_sum={}", sample1_sum);
    
    let mut sample2_sum = 0u64;
    sample2_sum += calc_card2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", &mut instances);
    sample2_sum += calc_card2("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", &mut instances);
    sample2_sum += calc_card2("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", &mut instances);
    sample2_sum += calc_card2("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", &mut instances);
    sample2_sum += calc_card2("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", &mut instances);
    sample2_sum += calc_card2("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", &mut instances);
    println!("sample2_sum={}", sample2_sum);

    let input = fs::read_to_string("input.txt").expect("");

    let mut sum1 = 0u32;
    for line in input.split('\n') {
        if line == "" {continue;}
        sum1 += calc_card(line);
    }
    println!("sum1={}", sum1);
    
    let mut sum2 = 0u64;
    instances = [1; MAX_INSTANCES];
    for line in input.split('\n') {
        if line == "" {continue;}
        sum2 += calc_card2(line, &mut instances);
    }
    println!("sum2={}", sum2);
}
