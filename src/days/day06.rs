use crate::common::Solution;
use std::collections::HashSet;

fn count_chars(txt: &String) -> usize {
    let mut counter = txt.chars().collect::<HashSet<_>>();
    counter.remove(&' ');
    counter.len()
}

fn count_dup_chars(txt: &String) -> usize {
    txt.split_whitespace()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .fold(None, |acc, hs| {
            acc.map(|a: HashSet<_>| a.intersection(&hs).map(|s| *s).collect())
                .or(Some(hs))
        })
        .unwrap()
        .len()
}

fn part1(input: &Vec<String>) -> String {
    input
        .iter()
        .map(|s| count_chars(s))
        .sum::<usize>()
        .to_string()
}

fn part2(input: &Vec<String>) -> String {
    input
        .iter()
        .map(|s| count_dup_chars(s))
        .sum::<usize>()
        .to_string()
}

// INPUT NEEDS TO BE PREFORMATTED -> 1 GROUP PER LINE, PEOPLE SEPARATED BY SPACES!
fn parse_input(raw_input: &[String]) -> Vec<String> {
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
