use crate::common::Solution;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fmt, str::FromStr, string::ParseError};

lazy_static! {
    static ref FIELDS: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        "cid".to_string(),
    ];
    static ref FOUR_DIG: Regex = Regex::new(r"^\d{4}$").unwrap();
    static ref NINE_DIG: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref HGT: Regex = Regex::new(r"^\d+(cm|in)$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref ECL: Regex = Regex::new(r"^(amb|b(lu|rn)|gr[yn]|hzl|oth)$").unwrap();
}

struct Passport {
    data: HashMap<String, String>,
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m: HashMap<String, String> = s
            .split_whitespace()
            .into_iter()
            .map(|x| {
                let mut xx = x.split(':');
                (xx.next().unwrap().to_owned(), xx.next().unwrap().to_owned())
            })
            .collect();

        Ok(Passport::new(m))
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")
    }
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
        let mut result = true;

        if let Some(byr) = self.data.get(&"byr".to_string()) {
            result &= self.verify_four_dig(byr, 1920, 2002);
        } else {
            return false;
        }

        if let Some(iyr) = self.data.get(&"iyr".to_string()) {
            result &= self.verify_four_dig(iyr, 2010, 2020);
        } else {
            return false;
        }

        if let Some(eyr) = self.data.get(&"eyr".to_string()) {
            result &= self.verify_four_dig(eyr, 2020, 2030);
        } else {
            return false;
        }

        if let Some(hgt) = self.data.get(&"hgt".to_string()) {
            result &= self.verify_hgt(hgt);
        } else {
            return false;
        }

        if let Some(hcl) = self.data.get(&"hcl".to_string()) {
            result &= HCL.is_match(hcl);
        } else {
            return false;
        }
        if let Some(ecl) = self.data.get(&"ecl".to_string()) {
            result &= ECL.is_match(ecl);
        } else {
            return false;
        }

        if let Some(pid) = self.data.get(&"pid".to_string()) {
            result &= NINE_DIG.is_match(pid);
        } else {
            return false;
        }

        result
    }

    fn is_valid(&self) -> bool {
        let proper_fields = self
            .data
            .iter()
            .map(|(k, _)| k)
            .filter(|k| FIELDS.contains(k))
            .count();
        proper_fields == 8 || (proper_fields == 7 && !self.data.contains_key("cid"))
    }
}

fn part1(input: &Vec<Passport>) -> String {
    input.iter().filter(|x| x.is_valid()).count().to_string()
}

fn part2(input: &Vec<Passport>) -> String {
    input
        .iter()
        .filter(|x| x.validate_fields())
        .count()
        .to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<Passport> = raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect();

    (part1(&input), part2(&input))
}
