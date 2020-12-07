use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref HGT: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

static ECL: &[&str; 7] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn new(data: &str) -> Option<Passport> {
        let dict: HashMap<&str, &str> = data
            .split(|c| c == ' ' || c == '\n')
            .filter_map(|s| {
                let mut kv = s.split(":");
                Some((kv.next().unwrap(), kv.next().unwrap()))
            })
            .collect();

        return Some(Passport {
            byr: dict.get("byr")?.parse().unwrap(),
            iyr: dict.get("iyr")?.parse().unwrap(),
            eyr: dict.get("eyr")?.parse().unwrap(),
            hgt: dict.get("hgt")?.parse().unwrap(),
            hcl: dict.get("hcl")?.parse().unwrap(),
            ecl: dict.get("ecl")?.parse().unwrap(),
            pid: dict.get("pid")?.parse().unwrap(),
        });
    }

    fn validate(&self) -> bool {
        self.validate_byr()
            && self.validate_ecl()
            && self.validate_hcl()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_pid()
            && self.validate_iyr()
    }

    fn validate_byr(&self) -> bool {
        self.byr >= 1920 && self.byr <= 2002
    }

    fn validate_iyr(&self) -> bool {
        self.iyr >= 2010 && self.iyr <= 2020
    }

    fn validate_eyr(&self) -> bool {
        self.eyr >= 2020 && self.eyr <= 2030
    }

    fn validate_hgt(&self) -> bool {
        match HGT.captures(&self.hgt) {
            None => false,
            Some(captures) => match (u32::from_str(&captures[1]), &captures[2]) {
                (Ok(height), "cm") => height >= 150 && height <= 193,
                (Ok(height), "in") => height >= 59 && height <= 76,
                _ => false,
            },
        }
    }

    fn validate_hcl(&self) -> bool {
        match HCL.captures(&self.hcl) {
            None => false,
            Some(_) => true,
        }
    }

    fn validate_ecl(&self) -> bool {
        ECL.contains(&&*self.ecl)
    }

    fn validate_pid(&self) -> bool {
        PID.is_match(&self.pid)
    }
}

fn main() {
    let data = fs::read_to_string("./examples/input/day-04.txt").unwrap();

    let passports: Vec<Passport> = data
        .split("\n\n")
        .map(|passport| Passport::new(passport))
        .filter(|passport| passport.is_some())
        .map(|passport| passport.unwrap())
        .collect();

    println!("Part 1: {}", passports.iter().count());

    println!(
        "Part 2: {}",
        passports
            .iter()
            .filter(|passport| passport.validate())
            .count()
    )
}
