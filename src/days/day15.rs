use crate::common::Solution;
use std::collections::HashMap;

fn find_nth_van_eck(input: &Vec<u32>, limit: usize) -> u32 {
    //for some reason, creating ::with_capacity() even with correct capacity makes this solution 2x slower...no idea why .__.
    let mut num_to_age: HashMap<u32, usize> = HashMap::new();
    let mut counter = input.len();
    for (i, &x) in input.iter().take(counter - 1).enumerate() {
        num_to_age.insert(x, i + 1);
    }
    let mut last_num = *input.last().unwrap();
    while counter < limit {
        if let Some(&last) = num_to_age.get(&last_num) {
            let new_num = counter - last;
            num_to_age.insert(last_num, counter);
            last_num = new_num as u32;
        } else {
            num_to_age.insert(last_num, counter);
            last_num = 0;
        }
        counter += 1;
    }

    last_num
}

fn part1(input: &InputType) -> String {
    find_nth_van_eck(input, 2020).to_string()
}

fn part2(input: &InputType) -> String {
    find_nth_van_eck(input, 30_000_000).to_string()
}

type InputType = Vec<u32>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .flat_map(|x| x.split(","))
        .map(|x| x.parse().unwrap())
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
