use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("./examples/input/day-06.txt").unwrap();

    let group_answers: Vec<&str> = input.split("\n\n").collect();

    let result = solve_part_01(&group_answers);
    println!("{}", result);

    let result = solve_part_02(&group_answers);
    println!("{}", result);
}

fn solve_part_01(group_answers: &Vec<&str>) -> usize {
    let mut dict: HashSet<char> = HashSet::new();

    let mut result: usize = 0;
    for answers in group_answers {
        for answer in answers.chars() {
            if answer != '\n' {
                dict.insert(answer);
            }
        }
        result += dict.len();
        dict.clear();
    }
    result
}

fn solve_part_02(group_answers: &Vec<&str>) -> usize {
    let mut dict: HashMap<char, u32> = HashMap::new();
    let mut result: usize = 0;

    for group_answer in group_answers {
        for answer in group_answer.chars() {
            if answer != '\n' {
                let counter: &mut u32 = dict.entry(answer).or_insert(0);
                *counter += 1;
            }
        }

        for value in dict.values() {
            if *value == group_answer.lines().count() as u32 {
                result += 1;
            }
        }

        dict.clear();
    }
    result
}
