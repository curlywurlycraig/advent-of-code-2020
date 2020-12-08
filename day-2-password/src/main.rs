use parse_display::{Display, FromStr};

mod input;
use input::input;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{lowest}-{highest} {character}: {password}")]
struct PasswordValidator {
    lowest: usize,
    highest: usize,
    character: char,
    password: String
}

impl PasswordValidator {
    fn is_valid(&self) -> bool {
        let count = self.password
            .chars()
            .filter(|c| *c == self.character)
            .collect::<Vec<char>>()
            .len();

        count >= self.lowest && count <= self.highest
    }

    fn is_valid_two(&self) -> bool {
        let first_char = self.password.chars().nth(self.lowest - 1).unwrap();
        let second_char = self.password.chars().nth(self.highest - 1).unwrap();

        (first_char == self.character) ^ (second_char == self.character)
    }
}


fn main() {
    let result = input
        .iter()
        .filter(|i| i.parse::<PasswordValidator>().expect("Failed to parse line.").is_valid())
        .count();

    println!("Result: {}", result);

    let result = input
        .iter()
        .filter(|i| i.parse::<PasswordValidator>().expect("Failed to parse line.").is_valid_two())
        .count();

    println!("Second result: {}", result);
}
