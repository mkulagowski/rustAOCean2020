use crate::common::Solution;
use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

#[derive(Clone)]
struct Combat {
    player1: VecDeque<u8>,
    player2: VecDeque<u8>,
    seen_states: HashSet<u64>,
}

impl Combat {
    fn new(p1: VecDeque<u8>, p2: VecDeque<u8>) -> Combat {
        Combat {
            player1: p1,
            player2: p2,
            seen_states: HashSet::new(),
        }
    }
    
    fn is_finished(&self) -> bool {
        self.player1.is_empty() || self.player2.is_empty()
    }

    fn get_winner(&self) -> &VecDeque<u8> {
        if self.player1.is_empty() {
            &self.player2
        } else {
            &self.player1
        }
    }

    fn save_state(&mut self) -> bool {
        let mut hasher = DefaultHasher::new();
        self.player1.hash(&mut hasher);
        self.seen_states.insert(hasher.finish())
    }

    fn sub_game_needed(&self, p1: u8, p2: u8) -> bool {
        p1 as usize <= self.player1.len() && p2 as usize <= self.player2.len()
    }

    fn play(&mut self) {
        while !self.is_finished() {
            let p1 = self.player1.pop_front().unwrap();
            let p2 = self.player2.pop_front().unwrap();

            if p1 > p2 {
                self.player1.push_back(p1);
                self.player1.push_back(p2);
            } else {
                self.player2.push_back(p2);
                self.player2.push_back(p1);
            }
        }
    }

    fn play_recursive(&mut self) -> bool {
        while !self.is_finished() && self.save_state() {
            let p1 = self.player1.pop_front().unwrap();
            let p2 = self.player2.pop_front().unwrap();

            let has_p1_won = {
                if self.sub_game_needed(p1, p2) {
                    let mut sub_game = Combat::new(
                        self.player1.iter().copied().take(p1 as usize).collect(),
                        self.player2.iter().copied().take(p2 as usize).collect(),
                    );
                    sub_game.play_recursive()
                } else {
                    p1 > p2
                }
            };

            if has_p1_won {
                self.player1.push_back(p1);
                self.player1.push_back(p2);
            } else {
                self.player2.push_back(p2);
                self.player2.push_back(p1);
            }
        }

        !(self.is_finished() && self.player1.is_empty())
    }
}

fn part1(input: &InputType) -> String {
    let mut game = input.clone();
    game.play();

    game.get_winner()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1, *x as u64))
        .fold(0u64, |acc, (i, x)| acc + (i * x))
        .to_string()
}

fn part2(input: &InputType) -> String {
    let mut game = input.clone();
    let res = game.play_recursive();

    let winner = if res { &game.player1 } else { &game.player2 };

    winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u64 + 1, *x as u64))
        .fold(0u64, |acc, (i, x)| acc + (i * x))
        .to_string()
}

type InputType = Combat;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut iter = raw_input.iter();
    let player1 = iter
        .by_ref()
        .skip(1)
        .take_while(|&x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let player2 = iter
        .by_ref()
        .skip(1)
        .take_while(|&x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();

    Combat::new(player1, player2)
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
