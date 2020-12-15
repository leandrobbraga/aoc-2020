use std::cmp::{max, min};
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./examples/input/day-11.txt").unwrap();
    let mut seat_map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    print_seat_map(&seat_map);
    solve_part_01(&mut seat_map);
}

fn print_seat_map(seat_map: &Vec<Vec<char>>) {
    seat_map
        .iter()
        .map(|line| {
            line.iter().map(|char| print!("{}", char)).for_each(drop);
            println!()
        })
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

                let adjacents = n_adjacent(i, j, &seat_map);

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

fn n_adjacent(i: usize, j: usize, seat_map: &Vec<Vec<char>>) -> u32 {
    let ilb: usize = max(0, i as i32 - 1) as usize;
    let iub: usize = min(seat_map.len(), (i as i32 + 2) as usize);

    let jlb: usize = max(0, j as i32 - 1) as usize;
    let jub: usize = min(seat_map[0].len(), (j as i32 + 2) as usize);

    let mut count: u32 = 0;

    for line in ilb..iub {
        for row in jlb..jub {
            if (line == i) && (row == j) {
                continue;
            }

            if seat_map[line][row] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn count_seats(seat_map: &Vec<Vec<char>>) -> u32 {
    seat_map
        .iter()
        .map(|line| line.iter().filter(|char| **char == '#').count() as u32)
        .sum()
}
