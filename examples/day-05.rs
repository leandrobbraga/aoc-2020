use std::fs;

fn main() {
    let result = fs::read_to_string("./examples/input/day-05.txt")
        .unwrap()
        .lines()
        .map(|line| parse(line))
        .max()
        .unwrap();

    println!("{}", result)
}

fn parse(line: &str) -> u32 {
    let mut lower_bound_row: f32 = 0.0;
    let mut upper_bound_row: f32 = 127.0;
    let mut lower_bound_col: f32 = 0.0;
    let mut upper_bound_col: f32 = 7.0;

    for char in line.chars() {
        if char == 'F' {
            upper_bound_row = ((upper_bound_row + lower_bound_row) / 2.0).floor()
        } else if char == 'B' {
            lower_bound_row = ((upper_bound_row + lower_bound_row) / 2.0).ceil()
        } else if char == 'L' {
            upper_bound_col = ((upper_bound_col + lower_bound_col) / 2.0).floor()
        } else {
            lower_bound_col = ((upper_bound_col + lower_bound_col) / 2.0).ceil()
        }
    }

    return (lower_bound_row * 8. + upper_bound_col) as u32;
}
