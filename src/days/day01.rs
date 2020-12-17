use crate::common::Solution;
use itertools::Itertools;
use std::collections::HashSet;

// P1: 600-800us, P2: 68ms
fn _find_combination_of(input: &Vec<i32>, comb_size: usize) -> i32 {
    for vals in input.into_iter().copied().combinations(comb_size) {
        if vals.iter().sum::<i32>() == 2020 {
            return vals.iter().product();
        }
    }
    0
}

// P1: 7-10us
fn _find_combination_of1(input: &Vec<i32>, _: usize) -> i32 {
    for (idx, val) in input.iter().enumerate() {
        for val2 in input.iter().skip(idx + 1) {
            if val + val2 == 2020 {
                return val * val2;
            }
        }
    }
    0
}

// P1: 6-8us
fn find_combination_of2<'a, I>(input_iter: I, target: i32) -> Option<i32>
where
    I: Iterator<Item = &'a i32>,
{
    let mut complements: HashSet<i32> = HashSet::new();
    for val in input_iter {
        let complement = target - val;
        if complements.contains(&complement) {
            return Some(complement * val);
        }
        complements.insert(*val);
    }
    None
}

// P2: 600-700us
fn find_combination_of3(input: &Vec<i32>, target: i32) -> i32 {
    for (idx, val) in input.iter().enumerate() {
        if let Some(prod) = find_combination_of2(input.iter().skip(idx + 1), target - val) {
            return prod * val;
        }
    }

    0
}

fn part1(input: &InputType) -> String {
    find_combination_of2(input.iter(), 2020)
        .unwrap()
        .to_string()
}

fn part2(input: &InputType) -> String {
    find_combination_of3(input, 2020).to_string()
}

type InputType = Vec<i32>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
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
