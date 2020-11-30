pub mod day01;

pub fn all_numbers() -> Vec<u8> {
    (1..=25).filter(|&day| get_solver(day).is_some()).collect()
}

pub fn get_solver(day: u8) -> Option<fn(&[String]) -> crate::common::Solution> {
    match day {
        1 => Some(day01::solve),
        _ => None,
    }
}
