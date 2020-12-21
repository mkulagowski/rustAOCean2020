use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref TITLE_REGX: Regex = Regex::new(r"Tile (\d+):").unwrap();
}
struct Tile {
    id: u32,
    _data: Vec<Vec<bool>>,
    borders: Vec<Vec<u16>>,
}

fn part1(input: &InputType) -> String {
    let mut border_counter: HashMap<u16, u8> = HashMap::new();
    input.iter().for_each(|x| {
        x.borders.iter().flatten().for_each(|x| {
            border_counter
                .entry(*x)
                .and_modify(|y| *y += 1)
                .or_insert(1);
        })
    });

    let corner_match_count = {
        let all_sides = 4;
        let all_sides_inv = all_sides;
        let matched = 2;
        let matched_inv = matched;
        all_sides + all_sides_inv + matched + matched_inv
    };

    input
        .iter()
        .map(|x| {
            (
                x.id,
                x.borders
                    .iter()
                    .flatten()
                    .filter_map(|b| border_counter.get(b))
                    .sum::<u8>(),
            )
        })
        .filter(|(_, x)| *x == corner_match_count)
        .map(|(i, _)| i as u64)
        .product::<u64>()
        .to_string()
}

fn part2(_input: &InputType) -> String {
    "".to_string()
}

type InputType = Vec<Tile>;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut iter = raw_input.iter();
    let mut tiles = Vec::new();
    while let Some(line) = iter.next() {
        if !line.is_empty() {
            let id = reparse!(line, TITLE_REGX, u32).unwrap();
            let data: Vec<Vec<bool>> = iter
                .by_ref()
                .take(10)
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect();
            let mut borders = Vec::new();
            let mut norms = Vec::new();
            norms.push(data[0].clone());
            norms.push(data[9].clone());
            norms.push(data.iter().map(|x| x[0]).collect());
            norms.push(data.iter().map(|x| x[9]).collect());
            borders.push(
                norms
                    .iter()
                    .map(|x| {
                        let mut res = 0u16;
                        for i in x.iter().rev() {
                            res <<= 1;
                            if *i {
                                res |= 1;
                            }
                        }
                        res
                    })
                    .collect(),
            );
            borders.push(
                norms
                    .iter()
                    .map(|x| {
                        let mut res = 0u16;
                        for i in x.iter() {
                            res <<= 1;
                            if *i {
                                res |= 1;
                            }
                        }
                        res
                    })
                    .collect(),
            );

            tiles.push(Tile {
                id,
                _data: data,
                borders,
            });
        }
    }
    tiles
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
