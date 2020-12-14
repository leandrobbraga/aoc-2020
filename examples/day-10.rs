use std::fs;

fn main() {
    let input: String = fs::read_to_string("./examples/input/day-10.txt").unwrap();
    let mut adapters: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.sort();

    solve_part_01(&adapters)
}

fn solve_part_01(adapters: &Vec<u64>) {
    let mut ones: u64 = 0;
    let mut threes: u64 = 1; // We start at 1 because the difference to the device is always 3

    let mut jolts_difference;

    for i in 0..adapters.len() {
        if i == 0 {
            jolts_difference = adapters[i];
        } else {
            jolts_difference = adapters[i] - adapters[i - 1];
        }

        if jolts_difference == 1 {
            ones += 1;
        } else if jolts_difference == 3 {
            threes += 1;
        }
    }

    println!("ones:{}, threes:{}, mul:{}", ones, threes, ones * threes)
}
