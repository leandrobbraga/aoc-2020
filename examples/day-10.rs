use std::cmp::max;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./examples/input/day-10.txt").unwrap();
    let mut adapters: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    adapters.push(adapters[adapters.len() - 1] + 3);

    solve_part_01(&adapters);
    solve_part_02(&adapters);
}

fn solve_part_01(adapters: &[u64]) {
    let mut ones: u64 = 0;
    let mut threes: u64 = 0;

    let mut jolts_difference;

    for i in 1..adapters.len() {
        jolts_difference = adapters[i] - adapters[i - 1];

        if jolts_difference == 1 {
            ones += 1;
        } else if jolts_difference == 3 {
            threes += 1;
        }
    }

    println!(
        "Part 01 - ones:{}, threes:{}, mul:{}",
        ones,
        threes,
        ones * threes
    )
}

fn solve_part_02(adapters: &[u64]) {
    let mut paths: Vec<u64> = vec![];

    paths.push(1);

    for i in 1..adapters.len() {
        paths.push(0);
        let lb = max(0, i as i64 - 3) as usize;
        for j in lb..i {
            if adapters[i] - adapters[j] <= 3 {
                paths[i] += paths[j];
            }
        }
    }

    println!("Part 02 - Paths: {}", paths[adapters.len() - 1])
}
