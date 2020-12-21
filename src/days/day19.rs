use crate::common::Solution;
use std::collections::HashSet;
use std::{collections::HashMap, str::FromStr};

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
    idx: usize,
}

impl MsgDecoder {
    fn cc(&mut self, x: &Rule, inp: &Vec<u8>, ruleset: &HashMap<u8, RuleOp>, idx: usize) -> bool {
        if self.idx == inp.len() {
            return false;
        }

        match x {
            Rule::Value(c) => {
                let matching = inp[self.idx] == c.as_bytes()[0];
                if matching {
                    self.idx += 1;
                }
                matching
            }
            Rule::Pointers(ptrs) => {
                let curr_idx = self.idx;
                for p in ptrs {
                    if !self.check(inp, *p, ruleset, idx + 1) {
                        self.idx = curr_idx;
                        return false;
                    }
                }
                true
            }
        }
    }

    fn check(
        &mut self,
        inp: &Vec<u8>,
        rule: u8,
        ruleset: &HashMap<u8, RuleOp>,
        idx: usize,
    ) -> bool {
        if self.idx == inp.len() {
            return false;
        }
        let r = ruleset.get(&rule).unwrap();
        let res = match r {
            RuleOp::Concat(x) => self.cc(x, inp, ruleset, idx),
            RuleOp::Or(x, y) => {
                if self.cc(y, inp, ruleset, idx) {
                    return true;
                } else {
                    return self.cc(x, inp, ruleset, idx);
                }
            }
        };
        res
    }

    fn lemme_see(&mut self, inp: &Vec<u8>, ruleset: &HashMap<u8, RuleOp>) -> bool {
        self.idx = 0;
        let res = self.check(inp, 0, ruleset, 0);
        res && self.idx == inp.len()
    }
}

fn part1(input: &mut InputType) -> String {
    let rs = input.ruleset.clone();
    let msgs = input.msgs.clone();
    msgs.iter()
        .map(|x| {
            let res = input.lemme_see(&x.as_bytes().into_iter().map(|y| *y).collect(), &rs);
            if res {}
            res
        })
        .filter(|x| *x)
        .count()
        .to_string()
}

fn part2(_input: &mut InputType) -> String {
    // let mut rs = input.ruleset.clone();
    // rs.insert(
    //     8,
    //     RuleOp::Or(Rule::Pointers(vec![42]), Rule::Pointers(vec![42, 8])),
    // );
    // rs.insert(
    //     11,
    //     RuleOp::Or(
    //         Rule::Pointers(vec![42, 31]),
    //         Rule::Pointers(vec![42, 11, 31]),
    //     ),
    // );
    // let msgs = input.msgs.clone();
    // msgs.iter()
    //     .map(|x| {
    //         let res = input.lemme_see(&x.as_bytes().into_iter().map(|y| *y).collect(), &rs);
    //         if res {}
    //         res
    //     })
    //     .filter(|x| *x)
    //     .count()
    //     .to_string()
    "".to_string()
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
        idx: 0,
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
