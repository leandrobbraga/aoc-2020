use std::fs;

fn main() {
    let input: String = fs::read_to_string("./examples/input/day-09.txt").unwrap();
    let numbers: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let wrong_number = solve_part_1(&numbers).unwrap();
    solve_part_2(&numbers, wrong_number)
}

fn solve_part_1(numbers: &[u64]) -> Option<u64> {
    let preamble: usize = 25;

    for (position, number) in numbers.iter().enumerate() {
        if position < preamble {
            continue;
        }

        if find_match(numbers, preamble, position) {
            continue;
        }

        println!("Part 1: {}", number);
        return Some(*number);
    }

    None
}

fn find_match(numbers: &[u64], preamble: usize, position: usize) -> bool {
    for i in (position - preamble)..position {
        for j in i..position {
            if (numbers[i] + numbers[j]) == numbers[position] {
                return true;
            }
        }
    }

    false
}

fn solve_part_2(numbers: &[u64], number: u64) {
    let mut n = 2;

    while n < numbers.len() {
        for i in 0..(numbers.len() - n) {
            let range = &numbers[i..(i + n)];

            if range.iter().sum::<u64>() == number {
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();

                println!("Part 2: {} (min) + {} (max) = {}", min, max, min + max)
            }
        }

        n += 1
    }
}
