use crate::{common::Solution, reparse};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SEAT_PATTERN: Regex = Regex::new(r"(.{7})(.{3})").unwrap();
}

struct Ticket {
    code: String,
    _row: String,
    _column: String,
}

fn _bst_code(code: &String, limits: (u32, u32), mapping: (char, char)) -> u32 {
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

fn shift_code(code: &String) -> u32 {
    let mut res = 0;
    for id in code.bytes() {
        res <<= 1;
        if id == b'B' || id == b'R' {
            res |= 0x1;
        }
    }
    res
}

// P1: 70us
fn _count_seat_id2(seat_code: &Ticket) -> u32 {
    let row = _bst_code(&seat_code._row, (0, 127), ('F', 'B'));
    let column = _bst_code(&seat_code._column, (0, 7), ('L', 'R'));

    row * 8 + column
}

// P1: 60us
fn count_seat_id(seat_code: &Ticket) -> u32 {
    shift_code(&seat_code.code)
}

// P2: 110us
fn _find_seat2(input: &Vec<Ticket>) -> u32 {
    input
        .iter()
        .map(|x| &x.code)
        .map(shift_code)
        .sorted()
        .tuple_windows()
        .find(|(before, after)| after - before == 2)
        .map(|(before, _)| before + 1)
        .unwrap()
}

// P2: 65us
fn find_seat(input: &Vec<Ticket>) -> u32 {
    let mut seats = [false; 128 * 8];
    input
        .iter()
        .map(|x| &x.code)
        .for_each(|x| seats[shift_code(x) as usize] = true);

    seats
        .iter()
        .enumerate()
        .skip_while(|(_, &x)| !x)
        .skip_while(|(_, &x)| x)
        .next()
        .unwrap()
        .0 as u32
}

fn part1(input: &InputType) -> String {
    input.iter().map(count_seat_id).max().unwrap().to_string()
}

fn part2(input: &InputType) -> String {
    find_seat(input).to_string()
}

type InputType = Vec<Ticket>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .map(|x| {
            let (_row, _column) = reparse!(x, SEAT_PATTERN, String, String).unwrap();
            Ticket {
                code: x.to_string(),
                _row,
                _column,
            }
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
