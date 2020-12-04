use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr, string::ParseError};

struct Passport {
    data: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PASSPORT: Regex = Regex::new(r"(\w+):(.+)").unwrap();
        }
        let m: HashMap<String, String> = s
            .split_whitespace()
            .into_iter()
            .map(|x| reparse!(x, PASSPORT, String, String).unwrap())
            .collect();

        Ok(Passport::new(m))
    }
}

lazy_static! {
    static ref FIELDS: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
    ];
    static ref FOUR_DIG: Regex = Regex::new(r"^\d{4}$").unwrap();
    static ref NINE_DIG: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref HGT: Regex = Regex::new(r"^\d+(cm|in)$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref ECL: Regex = Regex::new(r"^(amb|b(lu|rn)|gr[yn]|hzl|oth)$").unwrap();
}

impl Passport {
    fn new(d: HashMap<String, String>) -> Passport {
        Passport { data: d }
    }

    fn verify_four_dig(&self, txt: &String, min: u32, max: u32) -> bool {
        if !FOUR_DIG.is_match(&txt) {
            return false;
        }
        let val = txt.parse::<u32>().unwrap();
        val >= min && val <= max
    }

    fn verify_hgt(&self, txt: &String) -> bool {
        if !HGT.is_match(&txt) {
            return false;
        }
        let mut mtxt = txt.clone();
        mtxt.truncate(txt.len() - 2);
        let val = mtxt.parse::<u32>().unwrap();
        match txt.ends_with("cm") {
            true => val >= 150 && val <= 193,
            false => val >= 59 && val <= 76,
        }
    }

    fn validate_fields(&self) -> bool {
        let valid_fields = self
            .data
            .iter()
            .filter(|&(key, val)| match key.as_str() {
                "byr" => self.verify_four_dig(val, 1920, 2002),
                "iyr" => self.verify_four_dig(val, 2010, 2020),
                "eyr" => self.verify_four_dig(val, 2020, 2030),
                "hgt" => self.verify_hgt(val),
                "hcl" => HCL.is_match(val),
                "ecl" => ECL.is_match(val),
                "pid" => NINE_DIG.is_match(val),
                _ => false,
            })
            .count();

        valid_fields == FIELDS.len()
    }

    fn is_valid(&self) -> bool {
        let proper_fields = self
            .data
            .iter()
            .map(|(k, _)| k)
            .filter(|k| FIELDS.contains(k))
            .count();
        proper_fields == FIELDS.len()
    }
}

fn part1(input: &Vec<Passport>) -> String {
    input.iter().filter(|&x| x.is_valid()).count().to_string()
}

fn part2(input: &Vec<Passport>) -> String {
    input
        .iter()
        .filter(|&x| x.validate_fields())
        .count()
        .to_string()
}

// INPUT NEEDS TO BE PREFORMATTED -> 1 PASSPORT PER LINE!
pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<Passport> = raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect();

    (part1(&input), part2(&input))
}
