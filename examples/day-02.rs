use std::fs::read_to_string;
use std::ops::RangeInclusive;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

struct PasswordPolicy {
    lower_bound: usize,
    upper_bound: usize,
    character: char,
    password: String,
}

impl PasswordPolicy {
    fn check_policy_1(&self) -> bool {
        let boundary: RangeInclusive<usize> = self.lower_bound..=self.upper_bound;

        boundary.contains(
            &self
                .password
                .chars()
                .filter(|c| c == &self.character)
                .count(),
        )
    }

    fn check_policy_2(&self) -> bool {
        (self.check_position(self.lower_bound) || self.check_position(self.upper_bound))
            && !(self.check_position(self.lower_bound) && self.check_position(self.upper_bound))
    }

    fn check_position(&self, position: usize) -> bool {
        return self.password.chars().nth(position - 1).unwrap() == self.character;
    }
}

fn main() {
    let file_contents: String = read_to_string("./examples/day-02.txt").unwrap();

    let password_policies = parse_password_policies(&file_contents);

    let result_1: usize = password_policies
        .iter()
        .map(|policy| policy.check_policy_1())
        .filter(|line| *line)
        .count();

    let result_2: usize = password_policies
        .iter()
        .map(|policy| policy.check_policy_2())
        .filter(|line| *line)
        .count();

    println!("part-1 result: {}", result_1);
    println!("part-2 result: {}", result_2);
}

fn parse_password_policies(password_policies: &String) -> Vec<PasswordPolicy> {
    password_policies
        .lines()
        .map(|policy| parse_password_policy(policy).unwrap())
        .collect()
}

fn parse_password_policy(password_policy: &str) -> Option<PasswordPolicy> {
    let result = RE.captures(password_policy).unwrap();

    Some(PasswordPolicy {
        lower_bound: result.get(1)?.as_str().parse().unwrap(),
        upper_bound: result.get(2)?.as_str().parse().unwrap(),
        character: result.get(3)?.as_str().parse().unwrap(),
        password: result.get(4)?.as_str().parse().unwrap(),
    })
}
