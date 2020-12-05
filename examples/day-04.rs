use std::collections::HashMap;
use std::fs;
use std::string::ParseError;

struct Password {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Password {
    fn parse(data: &str) -> Option<Password> {
        let mut dict: HashMap<&str, &str> = data
            .split(|c| c == ' ' || c == '\n')
            .filter_map(|s| {
                let mut kv = s.split(":");
                Some((kv.next().unwrap(), kv.next().unwrap()))
            })
            .collect();

        return Some(Password {
            byr: dict.get("byr")?.to_string(),
            iyr: dict.get("iyr")?.to_string(),
            eyr: dict.get("eyr")?.to_string(),
            hgt: dict.get("hgt")?.to_string(),
            hcl: dict.get("hcl")?.to_string(),
            ecl: dict.get("ecl")?.to_string(),
            pid: dict.get("pid")?.to_string(),
        });
    }
}

fn main() {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let data = fs::read_to_string("./examples/input/day-04.txt").unwrap();

    let count = data
        .split("\n\n")
        .filter(|passport| Password::parse(passport).is_some())
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
