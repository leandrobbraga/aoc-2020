use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./examples/input/day-06.txt").unwrap();

    let group_answers: Vec<&str> = input.split("\n\n").collect();

    let (result_part_01, result_part_02) = solve(&group_answers);
    println!("Part 01: {}", result_part_01);
    println!("Part 02: {}", result_part_02);
}

fn solve(group_answers: &Vec<&str>) -> (u32, u32) {
    let mut dict: HashMap<char, u32> = HashMap::new();
    let mut result_part_01: u32 = 0;
    let mut result_part_02: u32 = 0;

    for group_answer in group_answers {
        for answer in group_answer.chars() {
            if answer != '\n' {
                let counter: &mut u32 = dict.entry(answer).or_insert(0);
                *counter += 1;
            }
        }

        result_part_01 += dict.len() as u32;

        for value in dict.values() {
            if *value == group_answer.lines().count() as u32 {
                result_part_02 += 1;
            }
        }

        dict.clear();
    }
    (result_part_01, result_part_02)
}
