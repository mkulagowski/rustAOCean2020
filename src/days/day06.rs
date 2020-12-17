use crate::common::Solution;
use std::collections::HashSet;

// P1: 1000-1100us
fn _count_chars2(txt: &Vec<String>) -> usize {
    txt.iter()
        .flat_map(String::as_bytes)
        .collect::<HashSet<_>>()
        .len()
}

// P2: 2600-3000us
fn _count_dup_chars2(txt: &Vec<String>) -> usize {
    txt.iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .fold(None, |acc, hs| {
            acc.map(|a: HashSet<_>| a.intersection(&hs).map(|s| *s).collect())
                .or(Some(hs))
        })
        .unwrap()
        .len()
}

// P1,P2: 70-150us
fn count_chars(txt: &Vec<String>, limit: usize) -> usize {
    let mut char_counter = [0; 26];
    txt.iter()
        .flat_map(String::as_bytes)
        .for_each(|x| char_counter[(x - b'a') as usize] += 1);

    char_counter.iter().filter(|&&x| x >= limit).count()
}

fn part1(input: &InputType) -> String {
    input
        .iter()
        .map(|s| count_chars(s, 1))
        .sum::<usize>()
        .to_string()
}

fn part2(input: &InputType) -> String {
    input
        .iter()
        .map(|s| count_chars(s, s.len()))
        .sum::<usize>()
        .to_string()
}

// INPUT NEEDS TO BE PREFORMATTED -> 1 GROUP PER LINE, PEOPLE SEPARATED BY SPACES!
type InputType = Vec<Vec<String>>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .map(|x| x.split_whitespace().map(|s| s.to_string()).collect())
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
