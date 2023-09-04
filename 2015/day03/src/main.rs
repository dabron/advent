fn parse2(input: &str) -> u32 {
    const MAX: usize = 100;
    const SIZE: usize = MAX * 2 + 1;
    let mut houses: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut total = 1;
    let mut x: [usize; 2] = [MAX; 2];
    let mut y: [usize; 2] = [MAX; 2];
    houses[x[0]][y[0]] += 1;
    houses[x[1]][y[1]] += 1;
    let mut i: usize = 0;
    for c in input.chars() {
        i = (i + 1) % 2;
        match c {
            '>' => {x[i] += 1;},
            '<' => {x[i] -= 1;},
            '^' => {y[i] += 1;},
            'v' => {y[i] -= 1;},
            _ => {}
        }
        houses[x[i]][y[i]] += 1;
        if x[i] == 0 || x[i] == SIZE - 1 || y[i] == 0 || y[i] == SIZE - 1 {
            std::process::abort();
        }
        if houses[x[i]][y[i]] == 1 {
            total += 1;
        }
    }
    total
}

fn parse1(input: &str) -> u32 {
    const MAX: usize = 100;
    const SIZE: usize = MAX * 2 + 1;
    let mut houses: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut total = 1;
    let mut x: usize = MAX;
    let mut y: usize = MAX;
    houses[x][y] += 1;
    for c in input.chars() {
        match c {
            '>' => {x += 1;},
            '<' => {x -= 1;},
            '^' => {y += 1;},
            'v' => {y -= 1;},
            _ => {}
        }
        houses[x][y] += 1;
        if x == 0 || x == SIZE - 1 || y == 0 || y == SIZE - 1 {
            std::process::abort();
        }
        if houses[x][y] == 1 {
            total += 1;
        }
    }
    total
}

fn print(input: &str, total: u32) {
    println!("input={} total={}", input, total);
}

fn main() -> std::io::Result<()> {
    let mut input = ">";
    let mut total = parse1(&input);
    print(&input, total);
    input = "^v";
    total = parse2(&input);
    print(&input, total);

    input = "^>v<";
    total = parse1(&input);
    print(&input, total);
    total = parse2(&input);
    print(&input, total);

    input = "^v^v^v^v^v";
    total = parse1(&input);
    print(&input, total);
    total = parse2(&input);
    print(&input, total);

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    total = parse1(&buffer.as_str());
    println!("total1={}", total);

    total = parse2(&buffer.as_str());
    println!("total2={}", total);
    Ok(())
}
