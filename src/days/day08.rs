use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, str::FromStr};

lazy_static! {
    static ref CODE: Regex = Regex::new(r"(\w{3}) ([+-]\d+)").unwrap();
}

enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, arg) = reparse!(s, CODE, String, i32).unwrap();
        match instr.as_str() {
            "nop" => Ok(Instruction::Nop(arg)),
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            _ => Err(()),
        }
    }
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        match self {
            Instruction::Acc(val) => Instruction::Acc(*val),
            Instruction::Nop(val) => Instruction::Nop(*val),
            Instruction::Jmp(val) => Instruction::Jmp(*val),
        }
    }
}

struct CodeRunner {
    accumulator: usize,
    curr_idx: usize,
    data: Vec<Instruction>,
    visited_indices: HashSet<usize>,
    finished: bool,
}

impl CodeRunner {
    fn new(input: Vec<Instruction>) -> CodeRunner {
        CodeRunner {
            accumulator: 0,
            curr_idx: 0,
            data: input,
            visited_indices: HashSet::new(),
            finished: false,
        }
    }

    fn run(&mut self) -> (bool, usize) {
        while self.visited_indices.insert(self.curr_idx) {
            match self.data.get(self.curr_idx) {
                Some(Instruction::Acc(val)) => {
                    self.accumulator = (self.accumulator as i32 + val) as usize;
                    self.curr_idx += 1;
                }
                Some(Instruction::Jmp(offset)) => {
                    self.curr_idx = (self.curr_idx as i32 + offset) as usize
                }
                _ => self.curr_idx += 1,
            };

            if self.curr_idx >= self.data.len() {
                self.finished = true;
                break;
            }
        }
        (self.finished, self.accumulator)
    }

    fn _set(&mut self, idx: usize, val: Instruction) {
        if let Some(elem) = self.data.get_mut(idx) {
            *elem = val;
        }
    }

    fn _reset(&mut self) {
        self.finished = false;
        self.curr_idx = 0;
        self.accumulator = 0;
        self.visited_indices.clear();
    }
}

// my auto formatter kept splitting args here, so I shortened the types
type VecInstr = Vec<Instruction>;
type HSU = HashSet<usize>;
// P2: 45us
fn dfs(acc: usize, change: bool, idx: usize, data: &VecInstr, visited: &mut HSU) -> Option<usize> {
    let mut acc = acc;
    let mut idx = idx;
    while let Some(Instruction::Acc(val)) = data.get(idx) {
        if visited.insert(idx) {
            acc = (acc as i32 + val) as usize;
            idx += 1;
        } else {
            return None;
        }
    }

    if visited.insert(idx) {
        match data.get(idx) {
            Some(Instruction::Jmp(offset)) => {
                let offset_idx = (idx as i32 + offset) as usize;
                if let Some(x) = dfs(acc, change, offset_idx, data, visited) {
                    return Some(x);
                }
                if change {
                    return dfs(acc, false, idx + 1, data, visited);
                }
            }
            Some(Instruction::Nop(offset)) => {
                if let Some(x) = dfs(acc, change, idx + 1, data, visited) {
                    return Some(x);
                }
                if change {
                    let offset_idx = (idx as i32 + offset) as usize;
                    return dfs(acc, false, offset_idx, data, visited);
                }
            }
            None => return Some(acc),
            _ => {}
        }
    }
    None
}

// P2: 550-600us
fn _naive(input: &Vec<Instruction>) -> String {
    let mut runner = CodeRunner::new(input.to_vec());
    let mut res = runner.run();
    'outer: while !res.0 {
        for (i, inst) in input.iter().enumerate() {
            if let Instruction::Jmp(offset) = inst {
                runner._reset();
                runner._set(i, Instruction::Nop(*offset));
                res = runner.run();
                if res.0 {
                    break 'outer;
                }
                runner._set(i, Instruction::Jmp(*offset));
            }
        }

        for (i, inst) in input.iter().enumerate() {
            if let Instruction::Nop(offset) = inst {
                runner._reset();
                runner._set(i, Instruction::Jmp(*offset));
                res = runner.run();
                if res.0 {
                    break 'outer;
                }
                runner._set(i, Instruction::Nop(*offset));
            }
        }
    }

    res.1.to_string()
}

// 12-20us
fn part1(input: &InputType) -> String {
    let mut runner = CodeRunner::new(input.to_vec());
    runner.run().1.to_string()
}

fn part2(input: &InputType) -> String {
    let mut visited: HashSet<usize> = HashSet::new();
    let res = dfs(0, true, 0, input, &mut visited);
    res.unwrap().to_string()
}

type InputType = Vec<Instruction>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
