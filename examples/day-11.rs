use std::cmp::{max, min};
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./examples/input/day-11.txt").unwrap();
    let mut seat_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    solve_part_01(&mut seat_map);
    solve_part_02(&mut seat_map);
}

#[allow(dead_code)]
fn print_seat_map(seat_map: &[Vec<char>]) {
    seat_map
        .iter()
        .map(|line| println!("{}", line.iter().collect::<String>()))
        .for_each(drop)
}

fn solve_part_01(seat_map: &mut Vec<Vec<char>>) {
    let mut seat_map = seat_map.clone();
    let mut new_seat_map = seat_map.clone();
    let mut solved: bool = false;
    let mut seats = 0;

    while !solved {
        for i in 0..seat_map.len() {
            for j in 0..seat_map[0].len() {
                if seat_map[i][j] == '.' {
                    continue;
                }

                let adjacents = adjacent_seats(i, j, &seat_map);

                if adjacents == 0 {
                    new_seat_map[i][j] = '#';
                } else if adjacents >= 4 {
                    new_seat_map[i][j] = 'L';
                }
            }
        }
        if seat_map == new_seat_map {
            solved = true;
            seats = count_seats(&seat_map);
        }
        seat_map = new_seat_map.clone();
    }

    println!("Part 1: {}", seats)
}

fn adjacent_seats(i: usize, j: usize, seat_map: &[Vec<char>]) -> u32 {
    let ilb: usize = max(0, i as i32 - 1) as usize;
    let iub: usize = min(seat_map.len(), (i as i32 + 2) as usize);

    let jlb: usize = max(0, j as i32 - 1) as usize;
    let jub: usize = min(seat_map[0].len(), (j as i32 + 2) as usize);

    let mut count: u32 = 0;

    for (line, item) in seat_map.iter().enumerate().take(iub).skip(ilb) {
        for (col, value) in item.iter().enumerate().take(jub).skip(jlb) {
            if (line == i) && (col == j) {
                continue;
            }

            if *value == '#' {
                count += 1;
            }
        }
    }
    count
}

fn count_seats(seat_map: &[Vec<char>]) -> u32 {
    seat_map
        .iter()
        .map(|line| line.iter().filter(|char| **char == '#').count() as u32)
        .sum()
}

fn solve_part_02(seat_map: &mut Vec<Vec<char>>) {
    let mut seat_map = seat_map.clone();
    let mut new_seat_map = seat_map.clone();
    let mut solved: bool = false;
    let mut seats = 0;

    while !solved {
        for i in 0..seat_map.len() {
            for j in 0..seat_map[0].len() {
                if seat_map[i][j] == '.' {
                    continue;
                }

                let adjacents = seats_in_vision(i, j, &seat_map);

                if adjacents == 0 {
                    new_seat_map[i][j] = '#';
                } else if adjacents >= 5 {
                    new_seat_map[i][j] = 'L';
                }
            }
        }
        if seat_map == new_seat_map {
            solved = true;
            seats = count_seats(&seat_map);
        }
        seat_map = new_seat_map.clone();
    }

    println!("Part 2: {}", seats)
}

fn seats_in_vision(i: usize, j: usize, seat_map: &[Vec<char>]) -> u32 {
    let mut count: u32 = 0;

    let mut position: (i32, i32);

    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
        (1, -1),
        (-1, 1),
    ];

    let mut found;

    let map_size = (seat_map.len(), seat_map[0].len());

    for direction in directions {
        position = (i as i32, j as i32);

        found = false;

        position.0 += direction.0;
        position.1 += direction.1;

        while position.0 >= 0
            && position.1 >= 0
            && position.0 < map_size.0 as i32
            && position.1 < map_size.1 as i32
            && !found
        {
            if seat_map[position.0 as usize][position.1 as usize] == '#' {
                found = true;
                count += 1;
            } else if seat_map[position.0 as usize][position.1 as usize] == 'L' {
                found = true;
            }

            position.0 += direction.0;
            position.1 += direction.1;
        }
    }
    count
}
