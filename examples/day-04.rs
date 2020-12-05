use std::fs;

fn main() {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let count = fs::read_to_string("./examples/input/day-04.txt")
        .unwrap()
        .split("\n\n")
        .filter(|passport| check_passport(passport, &required_fields))
        .count();
    println!("{}", count);
}

fn check_passport(passport: &str, required_fields: &Vec<&str>) -> bool {
    for field in required_fields {
        if !passport.contains(field) {
            return false;
        }
    }
    return true;
}
