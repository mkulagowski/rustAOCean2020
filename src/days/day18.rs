use crate::{common::Solution, hashmap};
use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Ops {
    Add,
    Mul,
}

impl Ops {
    fn eval(&self, a: u64, b: u64) -> u64 {
        match self {
            Ops::Add => a + b,
            Ops::Mul => a * b,
        }
    }
}

impl FromStr for Ops {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Ops::Add),
            "*" => Ok(Ops::Mul),
            _ => Err(0),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Token {
    Value(u64),
    Op(Ops),
    LParens,
    RParens,
}

impl FromStr for Token {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Token::LParens),
            ")" => Ok(Token::RParens),
            op @ "*" | op @ "+" => Ok(Token::Op(op.parse().unwrap())),
            c @ _ => Ok(Token::Value(c.parse().unwrap())),
        }
    }
}

// Shunting-Yard
fn eval_onp(stack: Vec<Token>) -> u64 {
    let mut results: Vec<u64> = Vec::new();

    for t in stack {
        match t {
            Token::Op(op) => {
                let (a, b) = (results.pop().unwrap(), results.pop().unwrap());
                results.push(op.eval(a, b));
            }
            Token::Value(val) => results.push(val),
            _ => {}
        }
    }

    *results.first().unwrap()
}

fn build_onp(line: &Vec<Token>, priorities: &HashMap<Token, u8>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::with_capacity(line.len());
    let mut output: Vec<Token> = Vec::with_capacity(line.len());
    for &c in line {
        match c {
            tk @ Token::Value(_) => output.push(tk),
            tk @ Token::LParens => stack.push(tk),
            Token::RParens => {
                while let Some(top) = stack.pop() {
                    if top == Token::LParens {
                        break;
                    }
                    output.push(top);
                }
            }
            tk @ Token::Op(_) => {
                let prio = priorities.get(&tk);
                while let Some(top) = stack.last() {
                    let top_prio = priorities.get(top);
                    if top_prio < prio {
                        break;
                    }
                    output.push(stack.pop().unwrap());
                }
                stack.push(tk);
            }
        }
    }

    stack.drain(..).rev().for_each(|tk| output.push(tk));
    output
}

fn part1(input: &InputType) -> String {
    let priority_map: HashMap<Token, u8> = hashmap!(
        Token::LParens => 0,
        Token::RParens => 1,
        Token::Op(Ops::Mul) => 1,
        Token::Op(Ops::Add) => 1
    );

    input
        .iter()
        .fold(0u64, |acc, x| acc + eval_onp(build_onp(x, &priority_map)))
        .to_string()
}

fn part2(input: &InputType) -> String {
    let priority_map: HashMap<Token, u8> = hashmap!(
        Token::LParens => 0,
        Token::RParens => 1,
        Token::Op(Ops::Mul) => 1,
        Token::Op(Ops::Add) => 2
    );

    input
        .iter()
        .fold(0u64, |acc, x| acc + eval_onp(build_onp(x, &priority_map)))
        .to_string()
}

type InputType = Vec<Vec<Token>>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .map(|x| {
            x.chars()
                .into_iter()
                .filter(|x| x != &' ')
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
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
