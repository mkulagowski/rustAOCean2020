use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::AddAssign, str::FromStr};

lazy_static! {
    static ref MOVE_REGX: Regex = Regex::new(r"(\w)(\d+)").unwrap();
}

struct Coords {
    x: i64,
    y: i64,
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Coords {
    fn new(xx: i64, yy: i64) -> Self {
        Coords { x: xx, y: yy }
    }

    fn scale(&self, val: u32) -> Self {
        let scale = val as i64;
        Coords {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    fn manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }

    fn rotate(&self, angle: f64) -> Self {
        let rads = angle * std::f64::consts::PI / 180f64 * -1.0;
        let sinb = rads.sin();
        let cosb = rads.cos();

        let nx = cosb * self.x as f64 - sinb * self.y as f64;
        let ny = sinb * self.x as f64 + cosb * self.y as f64;
        Coords {
            x: (nx.round()) as i64,
            y: (ny.round()) as i64,
        }
    }
}

enum Direction {
    N,
    E,
    S,
    W,
    R,
    L,
    F,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //println!("match DIR: {}", s);
        match s {
            "N" => Ok(Direction::N),
            "E" => Ok(Direction::E),
            "S" => Ok(Direction::S),
            "W" => Ok(Direction::W),
            "R" => Ok(Direction::R),
            "L" => Ok(Direction::L),
            "F" => Ok(Direction::F),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn as_coords(&self) -> Coords {
        match self {
            Direction::N => Coords { x: 0, y: 1 },
            Direction::E => Coords { x: 1, y: 0 },
            Direction::S => Coords { x: 0, y: -1 },
            Direction::W => Coords { x: -1, y: 0 },
            _ => Coords { x: 0, y: 0 },
        }
    }

    fn as_u8(&self) -> u8 {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
            _ => panic!(),
        }
    }

    fn from_u8(dir: u8) -> Self {
        match dir & 0x3 {
            0 => Direction::N,
            1 => Direction::E,
            2 => Direction::S,
            3 => Direction::W,
            _ => panic!(),
        }
    }

    fn rotate(&self, times: i32) -> Self {
        let curr = self.as_u8();
        Direction::from_u8(curr + ((0x40 + times) as u8))
    }
}
struct Move {
    dir: Direction,
    val: u32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dirstr, val) = reparse!(s, MOVE_REGX, String, u32).unwrap();
        Ok(Move {
            dir: dirstr.parse::<Direction>().unwrap(),
            val,
        })
    }
}

fn part1(input: &Vec<Move>) -> String {
    let mut ship_pos = Coords::new(0, 0);
    let mut ship_dir = Direction::E;

    for m in input {
        match m.dir {
            Direction::N | Direction::E | Direction::S | Direction::W => {
                ship_pos += m.dir.as_coords().scale(m.val)
            }
            Direction::R => ship_dir = ship_dir.rotate(m.val as i32 / 90),
            Direction::L => ship_dir = ship_dir.rotate(m.val as i32 / 90 * -1),
            Direction::F => ship_pos += ship_dir.as_coords().scale(m.val),
        };
    }

    ship_pos.manhattan().to_string()
}

fn part2(input: &Vec<Move>) -> String {
    let mut ship_pos = Coords::new(0, 0);
    let mut ship_dir = Coords::new(10, 1);

    for m in input {
        match m.dir {
            Direction::N | Direction::E | Direction::S | Direction::W => {
                ship_dir += m.dir.as_coords().scale(m.val)
            }
            Direction::R => ship_dir = ship_dir.rotate(m.val as f64),
            Direction::L => ship_dir = ship_dir.rotate(m.val as f64 * -1.0),
            Direction::F => ship_pos += ship_dir.scale(m.val),
        };
    }

    ship_pos.manhattan().to_string()
}

fn parse_input(raw_input: &[String]) -> Vec<Move> {
    raw_input.iter().map(|x| x.parse().unwrap()).collect()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
