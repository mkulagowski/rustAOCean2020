use crate::common::Solution;
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
        let matches = PARSE_PATTERN.captures(s).unwrap();

        let min = matches
            .get(1)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let max = matches
            .get(2)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let character: char = matches
            .get(3)
            .map_or(' ', |m| m.as_str().chars().next().unwrap());
        let password: String = matches
            .get(4)
            .map_or("".to_string(), |m| m.as_str().to_string());

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

fn part1(input: &Vec<PasswordCheck>) -> String {
    input.iter().filter(|&x| x.check()).count().to_string()
}

fn part2(input: &Vec<PasswordCheck>) -> String {
    input
        .iter()
        .filter(|&x| x.check_slice())
        .count()
        .to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<PasswordCheck> = raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect();

    (part1(&input), part2(&input))
}
