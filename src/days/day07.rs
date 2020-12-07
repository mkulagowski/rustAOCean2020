use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use queues::*;
use regex::{Regex, RegexSet};
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref RULE_START: Regex =
        Regex::new(r"(\w+ \w+) bags? contain (\d+) (\w+ \w+) bags?").unwrap();
    static ref RULE_END: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    static ref RULE_EMPTY: Regex = Regex::new(r"(\w+ \w+) contain no other bags").unwrap();
    static ref REGS: RegexSet = RegexSet::new(&[
        r"(\w+ \w+) bags? contain (\d+) (\w+ \w+) bags?,",
        r"(\w+ \w+) bags? contain (\d+) (\w+ \w+) bags?."
    ])
    .unwrap();
}

static SEARCHED_BAG: &'static str = "shiny gold";

type RuleSetData = HashMap<String, Vec<(i32, String)>>;
type SimpleRuleSetData = HashMap<String, Vec<String>>;
struct RuleSet {
    data: RuleSetData,
}

enum RuleType {
    Multiple(),
    Single { key: String, qnt: i32, val: String },
    Empty(),
}

impl RuleType {
    //   light beige bags contain 5 dark green bags, 5 light gray bags, 3 faded indigo bags, 2 vibrant aqua bags.
    //   vibrant beige bags contain 1 pale silver bag.
    //   posh yellow bags contain no other bags.

    fn parse_rule(input: &String) -> RuleType {
        let matching_regx: Vec<usize> = REGS.matches(input.as_str()).into_iter().collect();
        match matching_regx.get(0) {
            Some(0) => RuleType::Multiple(),
            Some(1) => {
                let (key, qnt, val) =
                    reparse!(input.as_str(), RULE_START, String, i32, String).unwrap();
                RuleType::Single { key, qnt, val }
            }
            _ => RuleType::Empty(),
        }
    }
}

impl RuleSet {
    fn new() -> RuleSet {
        RuleSet {
            data: HashMap::new(),
        }
    }

    // Invert to a mapping of "bag type" => "bags that own it"
    fn invert_ruleset(&self) -> SimpleRuleSetData {
        let mut res: SimpleRuleSetData = HashMap::new();
        self.data.iter().for_each(|(key, vals)| {
            vals.into_iter().for_each(|(_, val)| {
                if !res.contains_key(val) {
                    res.insert(val.to_string(), vec![key.to_string()]);
                } else {
                    res.get_mut(val).unwrap().push(key.to_string());
                }
            })
        });

        res
    }

    fn add_rule(&mut self, input: &String) {
        match RuleType::parse_rule(input) {
            RuleType::Multiple() => {
                let mut split_rule = input.split(", ");
                let (node, qnt, sub_node) =
                    reparse!(split_rule.next().unwrap(), RULE_START, String, i32, String).unwrap();
                let mut vals: Vec<(i32, String)> = vec![(qnt, sub_node)];

                while let Some(txt) = split_rule.next() {
                    let (sqnt, snode) = reparse!(txt, RULE_END, i32, String).unwrap();
                    vals.push((sqnt, snode));
                }
                self.data.insert(node, vals);
            }
            RuleType::Single { key, qnt, val } => {
                self.data.insert(key, vec![(qnt, val)]);
            }
            _ => {}
        }
    }

    fn count_rules(&self, bag: &String) -> usize {
        if let Some(vals) = self.data.get(bag) {
            return vals
                .iter()
                .map(|(qnt, name)| (*qnt as usize, name))
                .map(|(qnt, name)| qnt + qnt * self.count_rules(name))
                .sum();
        }

        0usize
    }
}

fn part1(input: &RuleSet) -> String {
    let mut counter = 0usize;
    let mut visited: HashSet<String> = HashSet::new();
    let mut key_queue: Queue<String> = queue![SEARCHED_BAG.to_string()];
    let input = input.invert_ruleset();

    while key_queue.size() > 0 {
        let key = key_queue.remove().ok().unwrap();
        if let Some(val) = input.get(&key) {
            val.iter().for_each(|x| {
                if !visited.contains(x) {
                    key_queue.add(x.clone()).unwrap();
                    visited.insert(x.to_string());
                    counter += 1;
                }
            });
        }
    }

    counter.to_string()
}

fn part2(input: &RuleSet) -> String {
    input.count_rules(&SEARCHED_BAG.to_string()).to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let mut ruleset = RuleSet::new();
    raw_input.iter().for_each(|x| ruleset.add_rule(x));

    (part1(&ruleset), part2(&ruleset))
}
