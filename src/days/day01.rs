use crate::common::Solution;
use itertools::Itertools;

fn find_combination_of(input: &Vec<i32>, comb_size: usize) -> i32 {
    for vals in input.iter().copied().combinations(comb_size) {
        if vals.iter().sum::<i32>() == 2020 {
            return vals.iter().product();
        }
    }
    0
}

fn part1(input: &Vec<i32>) -> String {
    find_combination_of(input, 2).to_string()
}

fn part2(input: &Vec<i32>) -> String {
    find_combination_of(input, 3).to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<i32> = raw_input
        .into_iter()
        .map(|x| {
            x.parse()
                .unwrap_or_else(|x| panic!(format!("Could not parse value {}", x)))
        })
        .collect();

    (part1(&input), part2(&input))
}
