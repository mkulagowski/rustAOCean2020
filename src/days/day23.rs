use crate::common::Solution;

fn dec(num: u32, minn: u32, maxx: u32) -> u32 {
    if num == minn {
        return maxx;
    }
    num - 1
}
fn inc(num: usize) -> usize {
    if num == 8 {
        return 0;
    }
    num + 1
}

fn part1(input: &InputType) -> String {
    let mut input = input.clone();
    let mut curr_cup = 0;
    for m in 0..100 {
        let curr_val = input[curr_cup];
        let mut dest_val = dec(curr_val, 1, 9);
        // println!("-- move {} --", m + 1);
        // println!("cups: {:?}", input);
        // println!("current: {} [{}]", curr_val, curr_cup);
        if curr_cup >= 5 {
            input.rotate_left(curr_cup);
            curr_cup = 0;
        }
        let mut vals: Vec<u32> = input.drain(curr_cup + 1..curr_cup + 4).collect();

        //println!("pick up: {:?}", vals);

        while vals.contains(&dest_val) {
            dest_val = dec(dest_val, 1, 9);
        }
        let dest_cup = input
            .iter()
            .enumerate()
            .find(|&(_, x)| x == &dest_val)
            .map(|(i, _)| i)
            .unwrap();

        //println!("destination: {} [{}]\n", dest_val, dest_cup);
        let mut rest = input.split_off(dest_cup + 1);
        input.append(&mut vals);
        input.append(&mut rest);

        if dest_cup < curr_cup {
            curr_cup += 3;
        }
        curr_cup = inc(curr_cup);
    }

    let last_cup = input
        .iter()
        .enumerate()
        .find(|&(_, x)| x == &1)
        .map(|(i, _)| i)
        .unwrap();
    input.rotate_left(last_cup);
    input
        .iter()
        .skip(1)
        .flat_map(|x| x.to_string().chars().collect::<Vec<char>>())
        .collect::<String>()
}

fn part2(input: &InputType) -> String {
    let mut input = input.clone();
    let mut new_nums = (10..=1_000_000).into_iter().collect();
    input.append(&mut new_nums);
    let mut curr_cup = 0;
    for m in 0..10_000_000 {
        let curr_val = input[curr_cup];
        let mut dest_val = dec(curr_val, 1, 1_000_000);

        if curr_cup >= 5 {
            input.rotate_left(curr_cup);
            curr_cup = 0;
        }
        let mut vals: Vec<u32> = input.drain(curr_cup + 1..curr_cup + 4).collect();

        while vals.contains(&dest_val) {
            dest_val = dec(dest_val, 1, 1_000_000);
        }
        let dest_cup = input
            .iter()
            .enumerate()
            .find(|&(_, x)| x == &dest_val)
            .map(|(i, _)| i)
            .unwrap();

        let mut rest = input.split_off(dest_cup + 1);
        input.append(&mut vals);
        input.append(&mut rest);

        if dest_cup < curr_cup {
            curr_cup += 3;
        }
        curr_cup = inc(curr_cup);
    }

    let last_cup = input
        .iter()
        .enumerate()
        .find(|&(_, x)| x == &1)
        .map(|(i, _)| i)
        .unwrap();
    input.rotate_left(last_cup);
    input
        .iter()
        .skip(1)
        .take(2)
        .map(|x| {
            println!("res={}", x);
            *x as u64
        })
        .product::<u64>()
        .to_string()
}

type InputType = Vec<u32>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .iter()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
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
