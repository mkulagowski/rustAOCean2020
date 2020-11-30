use crate::common::Solution;

fn calculate_fuel(mass: i32) -> i32 {
    ((mass / 3) - 2).max(0)
}

fn part1(input: &Vec<i32>) -> String {
    let mut fuel = 0;
    for num in input {
        fuel += calculate_fuel(*num);
    }
    fuel.to_string()
}

fn part2(input: &Vec<i32>) -> String {
    let mut fuel = 0;
    for &num in input {
        let mut num = num;
        while num > 0 {
            num = calculate_fuel(num);
            fuel += num;
        }
    }
    fuel.to_string()
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
