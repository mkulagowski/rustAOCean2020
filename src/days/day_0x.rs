use crate::common::Solution;

fn part1(input: &Vec<String>) -> String {
    "".to_string()
}

fn part2(input: &Vec<String>) -> String {
    "".to_string()
}

fn parse_input(raw_input: &[String]) -> Vec<String> {
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
