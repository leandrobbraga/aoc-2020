use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./examples/input/day-06.txt").unwrap();

    let group_answers: Vec<&str> = input.split("\n\n").collect();

    let (result_part_01, result_part_02) = solve(&group_answers);
    println!("Part 01: {}", result_part_01);
    println!("Part 02: {}", result_part_02);
}

fn solve(groups_answers: &[&str]) -> (u32, u32) {
    let mut answers_counter: HashMap<char, u32> = HashMap::new();
    let mut result_part_01: u32 = 0;
    let mut result_part_02: u32 = 0;

    for group_answers in groups_answers {
        for answer in group_answers.chars() {
            if answer != '\n' {
                let answer_counter: &mut u32 = answers_counter.entry(answer).or_insert(0);
                *answer_counter += 1;
            }
        }

        result_part_01 += answers_counter.len() as u32;

        for value in answers_counter.values() {
            if *value == group_answers.lines().count() as u32 {
                result_part_02 += 1;
            }
        }

        answers_counter.clear();
    }
    (result_part_01, result_part_02)
}
