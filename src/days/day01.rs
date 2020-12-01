use crate::common::Solution;

fn part1(input: &Vec<i32>) -> String {
    for (idx1, val1) in input.iter().enumerate() {
        for val2 in input.iter().skip(idx1 + 1) {
            if val1 + val2 == 2020 {
                return (val1 * val2).to_string();
            }
        }
    }
    String::from("ERR")
}

fn part2(input: &Vec<i32>) -> String {
    for (idx1, val1) in input.iter().enumerate() {
        for (idx2, val2) in input.iter().skip(idx1 + 1).enumerate() {
            for val3 in input.iter().skip(idx2 + 1) {
                if val1 + val2 + val3 == 2020 {
                    return (val1 * val2 * val3).to_string();
                }
            }
        }
    }
    String::from("ERR")
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
