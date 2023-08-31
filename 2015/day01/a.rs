use std::io;

fn main() -> io::Result<()> {
    print_elevator("(())");
    print_elevator("()()");
    print_elevator("(((");
    print_elevator("(()(()(");
    print_elevator("))(((((");
    print_elevator("())");
    print_elevator("))(");
    print_elevator(")))");
    print_elevator(")())())");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    print_elevator(buffer.as_str());
    Ok(())
}

fn print_elevator(s: &str) {
    println!("input={0}, output={1}", s, elevator(s))
}

fn elevator(s: &str) -> i32 {
    let mut i = 0;
    for c in s.chars()
    {
        if c == '(' {
            i += 1
        }
        else if c == ')' {
            i -= 1
        }
    }
    i
}
