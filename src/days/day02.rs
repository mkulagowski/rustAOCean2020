use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::string::ParseError;

lazy_static! {
    static ref PARSE_PATTERN: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

struct PasswordCheck {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl FromStr for PasswordCheck {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max, character, password) =
            reparse!(s, PARSE_PATTERN, usize, usize, char, String).unwrap();

        Ok(PasswordCheck {
            min,
            max,
            character,
            password,
        })
    }
}

impl PasswordCheck {
    fn check(&self) -> bool {
        let count = self.password.matches(self.character).count();
        count >= self.min && count <= self.max
    }

    fn check_slice(&self) -> bool {
        let from = self.min - 1;
        let to = self.max - self.min - 1;
        let mut pass_chars = self.password.chars();
        let first_matches = pass_chars.nth(from).unwrap() == self.character;
        let second_matches = pass_chars.nth(to).unwrap() == self.character;
        first_matches ^ second_matches
    }
}

fn part1(input: &InputType) -> String {
    input.iter().filter(|&x| x.check()).count().to_string()
}

fn part2(input: &InputType) -> String {
    input
        .iter()
        .filter(|&x| x.check_slice())
        .count()
        .to_string()
}

type InputType = Vec<PasswordCheck>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
