use crate::common::Solution;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashSet;

static PART1_SOLUTION: usize = 542529149;
static PRE_SIZE: usize = 25;

fn find_combination_of2<'a, I>(input_iter: I, target: usize) -> Option<usize>
where
    I: Iterator<Item = &'a usize>,
{
    let mut complements: HashSet<usize> = HashSet::new();
    for val in input_iter {
        let complement = target - val;
        if complements.contains(&complement) {
            return Some(complement * val);
        }
        complements.insert(*val);
    }
    None
}

fn part1(input: &InputType) -> String {
    let mut preamble = input.iter().copied().take(PRE_SIZE).collect::<HashSet<_>>();

    input
        .iter()
        .enumerate()
        .skip(PRE_SIZE)
        .map(|(idx, &curr)| {
            let comb = find_combination_of2(preamble.iter(), curr);
            let pre_start = input.get(idx - PRE_SIZE).unwrap();
            preamble.insert(curr);
            preamble.remove(pre_start);
            (curr, comb)
        })
        .find(|(_, comb)| comb.is_none())
        .unwrap()
        .0
        .to_string()
}

fn part2(input: &InputType) -> String {
    let mut acc: usize = 0;
    let mut start_idx = 0;
    for (i, x) in input.iter().enumerate() {
        if acc > PART1_SOLUTION {
            while acc > PART1_SOLUTION {
                acc -= input.get(start_idx).unwrap();
                start_idx += 1;
            }
        }

        if acc == PART1_SOLUTION {
            if let MinMax(mn, mx) = input.iter().skip(start_idx).take(i - start_idx).minmax() {
                return (mx + mn).to_string();
            }
        }

        acc += x;
    }

    "".to_string()
}

type InputType = Vec<usize>;
fn parse_input(raw_input: &[String]) -> InputType {
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
