use crate::common::Solution;
use std::{collections::HashMap, str::FromStr};
use regex::Regex;

#[derive(Debug, Clone)]
enum RuleOp {
    Concat(Rule),
    Or(Rule, Rule),
}
#[derive(Debug, Clone)]
enum Rule {
    Value(String),
    Pointers(Vec<u8>),
}

impl FromStr for Rule {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss: Vec<u8> = s
            .split_whitespace()
            .into_iter()
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(Rule::Pointers(ss))
    }
}

struct MsgDecoder {
    ruleset: HashMap<u8, RuleOp>,
    msgs: Vec<String>,
}

impl MsgDecoder {
    fn concat_rules(&self, x: &Rule) -> String {
        match x {
            Rule::Value(c) => c.to_string(),
            Rule::Pointers(ptrs) => {
                let mut res = String::new();
                for p in ptrs {
                    res.push_str(self.get_regex(*p).as_str());
                }
                res
            }
        }
    }

    fn get_regex(&self, rule :u8) -> String {
        match self.ruleset.get(&rule).unwrap() {
            RuleOp::Concat(x) => self.concat_rules(x),
            RuleOp::Or(x, y) => format!("(?:{}|{})", self.concat_rules(x), self.concat_rules(y))
        }
    }
}

fn part1(input: &mut InputType) -> String {
    let regstr = format!("^{}$", input.get_regex(0));
    let reg = Regex::new(regstr.as_str()).unwrap();

    input.msgs.iter()
        .filter(|&x| reg.is_match(x))
        .count()
        .to_string()
}

fn part2(input: &mut InputType) -> String {
    let mut curr_idx = 255u8;
    input.ruleset.insert(curr_idx, RuleOp::Or(Rule::Pointers(vec![42]), Rule::Pointers(vec![42, 42])));
    input.ruleset.insert(curr_idx - 1, RuleOp::Or(Rule::Pointers(vec![42, 31]), Rule::Pointers(vec![42, 42, 31, 31])));
    curr_idx -= 2;
    for _ in 0..2 {
        input.ruleset.insert(curr_idx, RuleOp::Or(Rule::Pointers(vec![42]), Rule::Pointers(vec![42, curr_idx + 2])));
        input.ruleset.insert(curr_idx - 1, RuleOp::Or(Rule::Pointers(vec![42, 31]), Rule::Pointers(vec![42, curr_idx + 1, 31])));
        curr_idx -= 2;
    }
    input.ruleset.insert(8, RuleOp::Or(Rule::Pointers(vec![42]), Rule::Pointers(vec![42, curr_idx + 2])));
    input.ruleset.insert(11, RuleOp::Or(Rule::Pointers(vec![42, 31]), Rule::Pointers(vec![42, curr_idx + 1, 31])));

    let regstr = format!("^{}$", input.get_regex(0));
    let reg = Regex::new(regstr.as_str()).unwrap();
    input.msgs.iter()
        .filter(|&x| reg.is_match(x))
        .count()
        .to_string()
}

type InputType = MsgDecoder;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut iter = raw_input.iter();
    let mut ruleset: HashMap<u8, RuleOp> = HashMap::new();
    while let Some(line) = iter.next() {
        if !line.starts_with(|x: char| x.is_ascii_digit()) {
            break;
        }

        let (idx, rest) = {
            let mut s = line.split(':');
            (s.next().unwrap().parse().unwrap(), s.next().unwrap())
        };

        if line.contains('"') {
            let c: String = rest.trim().chars().skip(1).next().unwrap().to_string();
            ruleset.insert(idx, RuleOp::Concat(Rule::Value(c)));
        } else if rest.contains('|') {
            let (or1, or2) = {
                let mut s = rest.split('|');
                (s.next().unwrap(), s.next().unwrap())
            };
            ruleset.insert(idx, RuleOp::Or(or1.parse().unwrap(), or2.parse().unwrap()));
        } else {
            ruleset.insert(idx, RuleOp::Concat(rest.parse().unwrap()));
        }
    }

    let msgs = iter.map(|x| x.parse().unwrap()).collect();

    MsgDecoder {
        ruleset,
        msgs,
    }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let mut input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&mut input), part2(&mut input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
