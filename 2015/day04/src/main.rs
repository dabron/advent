use md5;

fn mine(input: &str, prefix: &str) -> u32 {
    let mut i = 0u32;
    loop {
        let input2 = format!("{}{}", input, i);
        let hash = md5::compute(input2);
        let hash_str = format!("{:x}", hash);
        if hash_str.starts_with(prefix) {
            break;
        }
        i += 1;
    }
    i
}

fn main() {
    let prefix = "00000";
    let input = "abcdef";
    let i = mine(input, prefix);
    println!("{}={}", input, i);

    let input = "pqrstuv";
    let i = mine(input, prefix);
    println!("{}={}", input, i);

    let input = "iwrupvqb";
    let i = mine(input, prefix);
    println!("{}={} '{}'", input, i, prefix);

    let prefix = "000000";
    let i = mine(input, prefix);
    println!("{}={} '{}'", input, i, prefix);
}
