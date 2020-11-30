use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub type Solution = (String, String);

pub fn day_input_filename(day: u8) -> PathBuf {
    let padded_day = format!("{:02}", day);
    Path::new("inputs").join(format!("day{}.in", padded_day))
}

pub fn get_input(path: &Path) -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_to_string(path)
        .expect(&format!("Input file not found: {:?}", path))
        .lines()
        .map(&str::to_string)
        .collect())
}

pub fn get_day_input(day: u8) -> Result<Vec<String>, std::io::Error> {
    get_input(&day_input_filename(day))
}
