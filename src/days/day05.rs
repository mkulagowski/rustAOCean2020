use crate::{common::Solution, reparse};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEAT_PATTERN: Regex = Regex::new(r"(.{7})(.{3})").unwrap();
}

fn bs_code(code: &String, limits: (u32, u32), mapping: (char, char)) -> u32 {
    let (mut min, mut max) = limits;
    let (lower, upper) = mapping;
    for id in code.chars() {
        match id {
            u if u == upper => min = (min + max) / 2 + 1,
            l if l == lower => max = (min + max) / 2,
            _ => {}
        }
    }
    max
}

fn count_seat_id(seat_code: &String) -> u32 {
    let (row_code, column_code) = reparse!(seat_code, SEAT_PATTERN, String, String).unwrap();
    let row = bs_code(&row_code, (0, 127), ('F', 'B'));
    let column = bs_code(&column_code, (0, 7), ('L', 'R'));

    row * 8 + column
}

fn part1(input: &Vec<String>) -> String {
    input.iter().map(count_seat_id).max().unwrap().to_string()
}

fn part2(input: &Vec<String>) -> String {
    input
        .iter()
        .map(count_seat_id)
        .sorted()
        .tuple_windows()
        .find(|(before, after)| after - before == 2)
        .map(|(before, _)| before + 1)
        .unwrap()
        .to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<String> = raw_input
        .iter()
        .map(|x| x.parse().expect(&format!("Could not parse value {}", x)))
        .collect();

    (part1(&input), part2(&input))
}
