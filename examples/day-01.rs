use std::fs::read_to_string;

fn main() {
    let file_contents: String =
        read_to_string("./examples/day-01.txt").expect("Could not read the file.");

    let numbers: Vec<u32> = file_contents
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let result_1 = solve_part_1(&numbers);
    println!("{}", result_1);

    let result_2 = solve_part_2(&numbers);
    println!("{}", result_2);
}

fn solve_part_1(numbers: &Vec<u32>) -> u32 {
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            }
        }
    }
    return 0;
}

fn solve_part_2(numbers: &Vec<u32>) -> u32 {
    for i in 0..numbers.len() {
        for j in i..numbers.len() {
            if numbers[i] + numbers[j] < 2020 {
                for k in j..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        return numbers[i] * numbers[j] * numbers[k];
                    }
                }
            }
        }
    }
    return 0;
}
