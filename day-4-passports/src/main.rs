use std::{fs::File, collections::HashSet};
use std::io::prelude::*;

const REQUIRED_KEYS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

fn is_passport_valid(passport_string: &str) -> bool {
    passport_string
        .trim()
        .split_ascii_whitespace()
        .map(|entry| entry.split(":").next().unwrap())
        .collect::<HashSet<&str>>()
        .into_iter()
        .filter(|key| REQUIRED_KEYS.contains(key))
        .count() == 7
}

fn main() {
    let mut file = File::open("src/input.txt").unwrap();
    let mut passports_string = String::new();
    file.read_to_string(&mut passports_string).unwrap();

    let valid_passport_count = passports_string.split("\n\n")
        .filter(|passport_string| is_passport_valid(&passport_string))
        .count();

    println!("There are {} valid passports", valid_passport_count);
}
