pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;

pub fn all_numbers() -> Vec<u8> {
    (1..=25).filter(|&day| get_solver(day).is_some()).collect()
}

pub fn get_solver(day: u8) -> Option<fn(&[String]) -> crate::common::Solution> {
    match day {
        1 => Some(day01::solve),
        2 => Some(day02::solve),
        3 => Some(day03::solve),
        4 => Some(day04::solve),
        5 => Some(day05::solve),
        6 => Some(day06::solve),
        7 => Some(day07::solve),
        8 => Some(day08::solve),
        9 => Some(day09::solve),
        10 => Some(day10::solve),
        11 => Some(day11::solve),
        12 => Some(day12::solve),
        13 => Some(day13::solve),
        14 => Some(day14::solve),
        _ => None,
    }
}
