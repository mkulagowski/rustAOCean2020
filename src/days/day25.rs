use crate::common::Solution;

fn transform(subject: u64, loop_size: usize) -> u64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val = (val * subject) % 20201227;
    }
    val
}

fn transform_until(subject: u64, result: u64) -> usize {
    let mut val = 1;
    let mut loop_size = 0;
    while val != result {
        val = (val * subject) % 20201227;
        loop_size += 1;
    }
    loop_size
}

fn part1(input: &InputType) -> String {
    let card_ls = transform_until(7, input.card_pub_key);
    let enc_key_card = transform(input.door_pub_key, card_ls);
    // let door_ls = transform_until(7, input.door_pub_key);
    // let enc_key_door = transform(input.card_pub_key, door_ls);
    // assert_eq!(enc_key_card, enc_key_door);
    enc_key_card.to_string()
}

fn part2(input: &InputType) -> String {
    "".to_string()
}

struct PubKeys {
    card_pub_key: u64,
    door_pub_key: u64,
}

type InputType = PubKeys;
fn parse_input(raw_input: &[String]) -> InputType {
    PubKeys {
        card_pub_key: raw_input[0].parse().unwrap(),
        door_pub_key: raw_input[1].parse().unwrap(),
    }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
