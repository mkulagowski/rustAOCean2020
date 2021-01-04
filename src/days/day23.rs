use crate::common::Solution;
use itertools::iterate;

fn create_next_list_small(input: &InputType) -> [usize; 10] {
    let mut next: [usize; 10] = [0; 10];
    input
        .windows(2)
        .for_each(|w| next[w[0] as usize] = w[1] as usize);
    next[0] = *input.first().unwrap() as usize;
    next[*input.last().unwrap() as usize] = next[0];
    next
}

unsafe fn fill_next_list_big(input: &InputType) {
    input
        .windows(2)
        .for_each(|w| BIG_LIST[w[0] as usize] = w[1] as usize);
    BIG_LIST[0] = *input.first().unwrap() as usize;

    let mut pp = *input.last().unwrap() as usize;
    let mut nn = input.len() + 1;
    while pp < BIG_LIST.len() - 1 {
        BIG_LIST[pp] = nn;
        pp = nn;
        nn += 1;
    }
    BIG_LIST[pp] = BIG_LIST[0];
}

fn do_da_crab(next: &mut [usize], moves_no: usize) {
    let mut c = 0;
    for _ in 0..moves_no {
        c = next[c];
        let p1 = next[c];
        let p2 = next[p1];
        let p3 = next[p2];
        let mut dst = c - 1;
        while dst == p1 || dst == p2 || dst == p3 || dst == 0 {
            if dst == 0 {
                dst = next.len() - 1;
            } else {
                dst -= 1;
            }
        }
        next[c] = next[p3];
        next[p3] = next[dst];
        next[dst] = p1;
    }
}

fn part1(input: &InputType) -> String {
    let mut next = create_next_list_small(&input);
    do_da_crab(&mut next, 100);

    iterate(next[1], |&c| next[c])
        .take_while(|&c| c != 1)
        .flat_map(|x| x.to_string().chars().collect::<Vec<char>>())
        .collect::<String>()
}

// remade solution so it uses static array, otherwise it would just overlow stack on my laptop
static mut BIG_LIST: [usize; 1_000_001] = [0; 1_000_001];

fn part2(input: &InputType) -> String {
    unsafe {
        fill_next_list_big(&input);
        do_da_crab(&mut BIG_LIST, 10_000_000);
        (BIG_LIST[1] * BIG_LIST[BIG_LIST[1]]).to_string()
    }
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
