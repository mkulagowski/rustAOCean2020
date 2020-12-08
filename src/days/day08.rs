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

    fn set(&mut self, idx: usize, val: Instruction) {
        if let Some(elem) = self.data.get_mut(idx) {
            *elem = val;
        }
    }

    fn reset(&mut self) {
        self.finished = false;
        self.curr_idx = 0;
        self.accumulator = 0;
        self.visited_indices.clear();
    }
}

fn part1(input: &Vec<Instruction>) -> String {
    let mut runner = CodeRunner::new(input.to_vec());
    runner.run().1.to_string()
}

fn part2(input: &Vec<Instruction>) -> String {
    let mut runner = CodeRunner::new(input.to_vec());
    let mut res = runner.run();
    'outer: while !res.0 {
        for (i, inst) in input.iter().enumerate() {
            if let Instruction::Jmp(offset) = inst {
                runner.reset();
                runner.set(i, Instruction::Nop(*offset));
                res = runner.run();
                if res.0 {
                    break 'outer;
                }
                runner.set(i, Instruction::Jmp(*offset));
            }
        }

        for (i, inst) in input.iter().enumerate() {
            if let Instruction::Nop(offset) = inst {
                runner.reset();
                runner.set(i, Instruction::Jmp(*offset));
                res = runner.run();
                if res.0 {
                    break 'outer;
                }
                runner.set(i, Instruction::Nop(*offset));
            }
        }
    }

    res.1.to_string()
}

fn parse_input(raw_input: &[String]) -> Vec<Instruction> {
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
