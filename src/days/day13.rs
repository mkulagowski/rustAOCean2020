use crate::common::Solution;

struct TimeTable {
    time: i64,
    bus_ids: Vec<(i64, i64)>,
}

fn modular_multiplicative_inverse(a: i64, m: i64) -> i64 {
    let b = a % m;
    if let Some(res) = (1..m).into_iter().find(|&x| (b * x) % m == 1) {
        return res;
    }

    1i64
}

fn chinese_remainder(input: &Vec<(i64, i64)>) -> i64 {
    let n_prod: i64 = input.iter().map(|&(_, x)| x).product();
    input
        .iter()
        .map(|&(a, n)| {
            let p = n_prod / n;
            let mmi = modular_multiplicative_inverse(p, n);
            (n - a) * mmi * p
        })
        .sum::<i64>()
        % n_prod
}

fn part1(input: &TimeTable) -> String {
    let (bus_id, wait_time) = input
        .bus_ids
        .iter()
        .map(|(_, x)| x)
        .map(|&x| (x, x - (input.time % x)))
        .min_by(|(_, x), (_, y)| x.cmp(y))
        .unwrap();
    (bus_id * wait_time).to_string()
}

fn part2(input: &TimeTable) -> String {
    chinese_remainder(&input.bus_ids).to_string()
}

fn parse_input(raw_input: &[String]) -> TimeTable {
    let mut input_iter = raw_input.iter();
    let time = input_iter.next().unwrap().parse().unwrap();
    let bus_ids = input_iter
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(x, y)| (x as i64, y.parse().unwrap()))
        .collect();
    TimeTable { time, bus_ids }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
