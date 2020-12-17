use crate::common::Solution;
use std::collections::{HashMap, HashSet};

fn part1(input: &InputType) -> String {
    let (ones, threes) = input.windows(2).fold((0, 0), |(o, t), window| {
        let diff = window[1] - window[0];
        if diff == 1 {
            return (o + 1, t);
        } else if diff == 3 {
            return (o, t + 1);
        }
        (o, t)
    });
    (ones * (threes + 1)).to_string()
}

fn part2(input: &InputType) -> String {
    let end = input.last().unwrap() + 3;
    let sett: HashSet<i32> = input.iter().copied().chain(vec![end]).collect();
    let mut paths_to_end: HashMap<i32, usize> = HashMap::new();
    paths_to_end.insert(end, 1);

    input
        .iter()
        .rev()
        .map(|&x| (x, [x + 3, x + 2, x + 1]))
        .for_each(|(x, nexts)| {
            nexts
                .iter()
                .filter(|&next| sett.contains(next))
                .for_each(|next| {
                    let new_val = *paths_to_end.get(next).unwrap();
                    paths_to_end
                        .entry(x)
                        .and_modify(|x| *x += new_val)
                        .or_insert(new_val);
                })
        });

    paths_to_end.get(&0).unwrap().to_string()
}

type InputType = Vec<i32>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let mut input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    input.sort();
    input.insert(0, 0);
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
