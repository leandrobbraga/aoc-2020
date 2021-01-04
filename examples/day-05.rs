use std::fs;

fn main() {
    let seat_ids: Vec<u32> = fs::read_to_string("./examples/input/day-05.txt")
        .unwrap()
        .lines()
        .map(|line| parse(line))
        .collect();

    let result_part_1 = seat_ids.iter().max().unwrap();

    println!("Part 1: {}", result_part_1);

    solve_part_02(&seat_ids)
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

    (lower_bound_row * 8.0 + lower_bound_col) as u32
}

fn solve_part_02(seat_ids: &[u32]) {
    let mut seat_ids = seat_ids.to_owned();
    seat_ids.sort_unstable();

    for (i, seat_id) in seat_ids.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if seat_id - seat_ids[i - 1] == 2 {
            println!("Part 2: {}", seat_id - 1)
        }
    }
}
