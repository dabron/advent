use std::io;

fn main() -> io::Result<()> {
    print_basement_position("(())");
    print_basement_position("()()");
    print_basement_position("(((");
    print_basement_position("(()(()(");
    print_basement_position("))(((((");
    print_basement_position("())");
    print_basement_position("))(");
    print_basement_position(")))");
    print_basement_position(")())())");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    print_basement_position(buffer.as_str());
    Ok(())
}

fn print_basement_position(s: &str) {
    println!("input={0}, output={1}", s, elevator(s))
}

fn elevator(s: &str) -> i32 {
    let mut floor = 0;
    let mut pos = 0;
    for c in s.chars()
    {
        pos += 1;
        if c == '(' {
            floor += 1
        }
        else if c == ')' {
            floor -= 1
        }
        if floor == -1 {
            return pos;
        }
    }
    -1
}
