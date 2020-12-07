use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("./examples/input/day-06.txt").unwrap();

    let group_answers: Vec<&str> = input.split("\n\n").collect();

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

    println!("{}", result)
}
