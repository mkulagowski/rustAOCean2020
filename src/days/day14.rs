use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MASK_REGX: Regex = Regex::new(r"mask = ([X01]+)").unwrap();
    static ref MEM_REGX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn part1(input: &Vec<BitmaskProg>) -> String {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    for prog in input {
        //println!("mask: {:?}\nvals={:?}\n", prog.masked_bits, prog.mem_inputs);
        prog.mem_inputs.iter().for_each(|&(i, x)| {
            let changed = prog.apply_mask(x);
            //println!("old={}, new={}, idx={}", x, changed, i);
            mem.insert(i, changed);
        });
    }

    mem.values().sum::<u64>().to_string()
}

fn part2(input: &Vec<BitmaskProg>) -> String {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    for prog in input {
        //println!("mask: {:?}\nvals={:?}\n", prog.masked_bits, prog.mem_inputs);
        prog.mem_inputs.iter().for_each(|&(i, x)| {
            //println!("mem[{}] = {}", i as u64, x);
            prog.generate_floatings(prog.apply_ones(i as u64))
                .iter()
                .for_each(|&ii| {
                    mem.insert(ii as usize, x);
                });
        });
    }

    mem.values().sum::<u64>().to_string()
}

struct BitmaskProg {
    masked_bits: Vec<(usize, u8)>,
    mem_inputs: Vec<(usize, u64)>,

    floating_bits: Vec<usize>,
    ones_mask: u64,
}

impl BitmaskProg {
    fn apply_mask(&self, val: u64) -> u64 {
        let mut toggle_mask = 0u64;
        for (i, x) in &self.masked_bits {
            //println!("check if bit{} of {:#b} is eq {}", *i, val, *x);
            if *x != BitmaskProg::check_bit(val, *i) {
                toggle_mask |= 0x1 << *i;
            }
            //println!("toggle mask: {:#b}", toggle_mask);
        }

        val ^ toggle_mask
    }

    fn apply_ones(&self, val: u64) -> u64 {
        val | self.ones_mask
    }

    fn generate_floatings(&self, val: u64) -> Vec<u64> {
        let mut res = vec![val];
        for ff in &self.floating_bits {
            let mut new_v: Vec<u64> = Vec::with_capacity(res.len() * self.floating_bits.len());
            //println!("res= {:?}", res);
            for rr in res {
                new_v.push(rr);
                new_v.push(rr ^ (0x1 << *ff));
            }
            //println!("res= {:?}", res);
            res = new_v;
        }
        res
    }

    fn check_bit(x: u64, i: usize) -> u8 {
        let res = ((x >> i) & 0x1) as u8;
        //println!("check_bit({}, {}) = {}", x, i, res);
        res
    }
}

fn parse_input(raw_input: &[String]) -> Vec<BitmaskProg> {
    let mut programes: Vec<BitmaskProg> = Vec::new();
    let mut masked_bits: Vec<(usize, u8)> = Vec::new();
    let mut mem_inputs: Vec<(usize, u64)> = Vec::new();
    let mut floating_bits: Vec<usize> = Vec::new();
    let mut ones_mask = 0u64;

    for line in raw_input {
        if line.starts_with("mask") {
            if !mem_inputs.is_empty() {
                programes.push(BitmaskProg {
                    masked_bits,
                    mem_inputs,
                    floating_bits,
                    ones_mask,
                });

                mem_inputs = Vec::new();
            }
            let mask_line = reparse!(line, MASK_REGX, String).unwrap();
            masked_bits = mask_line
                .as_bytes()
                .iter()
                .rev()
                .enumerate()
                .filter(|&(_, &x)| x != b'X')
                .map(|(i, &x)| (i, x - b'0'))
                .collect();
            ones_mask = masked_bits
                .iter()
                .fold(0u64, |mask, &(i, x)| mask | ((x as u64) << i));
            floating_bits = mask_line
                .as_bytes()
                .iter()
                .rev()
                .enumerate()
                .filter(|&(_, &x)| x == b'X')
                .map(|(i, _)| i)
                .collect();
        } else {
            mem_inputs.push(reparse!(line, MEM_REGX, usize, u64).unwrap());
        }
    }
    programes.push(BitmaskProg {
        masked_bits,
        mem_inputs,
        floating_bits,
        ones_mask,
    });

    programes
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
