use std::fs;

use lazy_static::lazy_static;
use queues::*;
use regex::Regex;
use std::collections::HashSet;

lazy_static! {
    static ref COLOR: Regex = Regex::new(r"^(.*) bags contain").unwrap();
    static ref CONTAINS: Regex = Regex::new(r"(\d+) (.*?) bags?").unwrap();
}

struct Rule {
    color: String,
    contains: Vec<Child>,
}

struct Child {
    color: String,
    amount: u32,
}

fn main() {
    let input = fs::read_to_string("./examples/input/day-07.txt").unwrap();
    let rules: Vec<Rule> = input.lines().map(|line| line_to_rule(line)).collect();

    println!("Part 1: {}", count_containers_part_1(&rules, "shiny gold"));
    println!("Part 2: {}", count_containers_part_2(&rules, "shiny gold"));
}

fn line_to_rule(unparsed_rule: &str) -> Rule {
    let color: &str = &COLOR.captures(unparsed_rule).unwrap()[1];

    let mut contains: Vec<Child> = vec![];

    for unparsed_contains in CONTAINS.captures_iter(unparsed_rule) {
        contains.push(Child {
            color: unparsed_contains[2].to_string(),
            amount: unparsed_contains[1].parse().unwrap(),
        });
    }

    Rule {
        color: color.to_string(),
        contains,
    }
}

fn count_containers_part_1(rules: &Vec<Rule>, color: &str) -> usize {
    let mut visited_bags: HashSet<String> = HashSet::new();
    let mut queue: Queue<String> = queue![];

    queue.add(color.to_string()).unwrap();

    while queue.size() > 0 {
        let next: String = queue.remove().unwrap();

        if !visited_bags.contains(&next) {
            let containers: Vec<(u32, String)> = find_containers(&rules, &next);
            for (_amount, color) in containers {
                queue.add(color).unwrap();
            }
        }

        visited_bags.insert(next);
    }

    return visited_bags.len() - 1;
}

fn find_containers(rules: &Vec<Rule>, color: &str) -> Vec<(u32, String)> {
    let mut containers: Vec<(u32, String)> = vec![];

    for rule in rules {
        for child in &rule.contains {
            if child.color == color {
                containers.push((child.amount, rule.color.clone()));
                break;
            }
        }
    }

    containers
}

fn count_containers_part_2(rules: &Vec<Rule>, color: &str) -> u32 {
    let mut result = 0;

    for rule in rules {
        if rule.color == color {
            for child in &rule.contains {
                result +=
                    child.amount + (child.amount * count_containers_part_2(&rules, &child.color));
            }
        }
    }

    result
}
