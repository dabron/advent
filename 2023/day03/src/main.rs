use std::fs;

const MAX_WIDTH: usize = 142;
const MAX_HEIGHT: usize = 142;

fn search1(grid: [[char; MAX_HEIGHT]; MAX_WIDTH], x: usize, y: usize, digits: usize) -> bool {
    let start = x - digits - 1;
    let end = x;
    for i in start..=end {
        if grid[i][y - 1] == '*' ||
           grid[i][y]     == '*' ||
           grid[i][y + 1] == '*' {
            return true;
        }
    }
    false
}

fn get_num(grid: [[char; MAX_HEIGHT]; MAX_WIDTH], x: usize, y: usize) -> (usize, usize, u32) {
    let mut i = x;
    let mut num = 0u32;
    let mut found = false;
    while grid[i][y].is_digit(10) {
        i -= 1;
        found = true;
    }
    if found {
        i += 1;
        loop {
            let c = grid[i][y];
            if c.is_digit(10) {
                num *= 10;
                num += c.to_digit(10).unwrap();
            }
            else {
                break;
            }
            i += 1;
        }
    }
    (i, y, num)
}

fn search2(grid: [[char; MAX_HEIGHT]; MAX_WIDTH], x: usize, y: usize) -> u32 {
    let mut a = 0u32;
    let mut b = 0u32;
    let mut ax: usize = 0;
    let mut ay: usize = 0;
    let mut bx: usize = 0;
    let mut by: usize = 0;
    
    if a == 0 {(ax, ay, a) = get_num(grid, x - 1, y - 1);}
    if a == 0 {(ax, ay, a) = get_num(grid, x    , y - 1);}
    if a == 0 {(ax, ay, a) = get_num(grid, x + 1, y - 1);}
    if a == 0 {(ax, ay, a) = get_num(grid, x - 1, y    );}
    if a == 0 {(ax, ay, a) = get_num(grid, x + 1, y    );}
    if a == 0 {(ax, ay, a) = get_num(grid, x - 1, y + 1);}
    if a == 0 {(ax, ay, a) = get_num(grid, x    , y + 1);}
    if a == 0 {(ax, ay, a) = get_num(grid, x + 1, y + 1);}

    if b == 0 {(bx, by, b) = get_num(grid, x + 1, y + 1);}
    if b == 0 {(bx, by, b) = get_num(grid, x    , y + 1);}
    if b == 0 {(bx, by, b) = get_num(grid, x - 1, y + 1);}
    if b == 0 {(bx, by, b) = get_num(grid, x + 1, y    );}
    if b == 0 {(bx, by, b) = get_num(grid, x - 1, y    );}
    if b == 0 {(bx, by, b) = get_num(grid, x + 1, y - 1);}
    if b == 0 {(bx, by, b) = get_num(grid, x    , y - 1);}
    if b == 0 {(bx, by, b) = get_num(grid, x - 1, y - 1);}

    println!("({}, {}): a={} ({}, {}), b={} ({}, {})", x, y, a, ax, ay, b, bx, by);

    if ax == bx && ay == by {
        println!("   only a single number!");
        return 0;
    }

    if a == b {
        println!("   double number!");
    }

    a * b
}

fn tally_grid(grid: [[char; MAX_HEIGHT]; MAX_WIDTH]) -> u32 {
    let mut digits: usize = 0;
    let mut num = 0u32;
    let mut tally = 0u32;

    for y in 0..MAX_HEIGHT {
        for x in 0..MAX_WIDTH {
            let c = grid[x][y];
            if c.is_digit(10) {
                digits += 1;
                num *= 10;
                num += c.to_digit(10).unwrap();
            }
            else {
                if num != 0 {
                    if search1(grid, x, y, digits) {
                        tally += num;
                    }
                    digits = 0;
                    num = 0;
                }
            }
        }
    }
    tally
}

fn tally_grid2(grid: [[char; MAX_HEIGHT]; MAX_WIDTH]) -> u32{
    let mut tally = 0u32;

    for y in 0..MAX_HEIGHT {
        for x in 0..MAX_WIDTH {
            let c = grid[x][y];
            if c == '*' {
                tally += search2(grid, x, y);
            }
        }
    }
    tally
}

fn parse_input1(input: &str) -> [[char; MAX_HEIGHT]; MAX_WIDTH] {
    let mut grid: [[char; MAX_HEIGHT]; MAX_WIDTH] = [['\0'; MAX_HEIGHT]; MAX_WIDTH];
    let mut x: usize = 1;
    let mut y: usize = 1;

    for c in input.chars() {
        if c == '\n' {
            x = 1;
            y += 1;
            continue;
        }
        else if c.is_digit(10) {
            grid[x][y] = c;
        }
        else if c != '.' {
            grid[x][y] = '*';
        }
        x += 1;
    }
    grid
}

fn parse_input2(input: &str) -> [[char; MAX_HEIGHT]; MAX_WIDTH] {
    let mut grid: [[char; MAX_HEIGHT]; MAX_WIDTH] = [['\0'; MAX_HEIGHT]; MAX_WIDTH];
    let mut x: usize = 1;
    let mut y: usize = 1;

    for c in input.chars() {
        if c == '\n' {
            x = 1;
            y += 1;
            continue;
        }
        else if c.is_digit(10) || c == '*' {
            grid[x][y] = c;
        }
        x += 1;
    }
    grid
}

fn main() {
    let sample = concat!(
        "467..114..\n",
        "...*......\n",
        "..35..633.\n",
        "......#...\n",
        "617*......\n",
        ".....+.58.\n",
        "..592.....\n",
        "......755.\n",
        "...$.*....\n",
        ".664.598..");
    let part_grid = parse_input1(sample);
    let part_tally = tally_grid(part_grid);
    println!("part tally: {}", part_tally);
    let gear_grid = parse_input2(sample);
    let gear_tally = tally_grid2(gear_grid);
    println!("gear tally: {}", gear_tally);

    let input = fs::read_to_string("input.txt").expect("");
    let grid1 = parse_input1(input.as_str());
    let tally1 = tally_grid(grid1);
    println!("tally1: {}", tally1);
    let grid2 = parse_input2(input.as_str());
    let tally2 = tally_grid2(grid2);
    println!("tally2: {}", tally2);
}
