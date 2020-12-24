use crate::common::Solution;
use std::{collections::HashSet, slice::Iter};
use std::{fmt, str::FromStr};

#[derive(Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::E),
            "se" => Ok(Direction::SE),
            "sw" => Ok(Direction::SW),
            "w" => Ok(Direction::W),
            "nw" => Ok(Direction::NW),
            "ne" => Ok(Direction::NE),
            _ => Err(()),
        }
    }
}

impl Direction {
    fn iter() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [
            Direction::E,
            Direction::SE,
            Direction::SW,
            Direction::W,
            Direction::NW,
            Direction::NE,
        ];
        DIRECTIONS.iter()
    }

    fn get_move(&self) -> Coords {
        match self {
            Direction::E => Coords::new(2, 0),
            Direction::SE => Coords::new(1, -2),
            Direction::SW => Coords::new(-1, -2),
            Direction::W => Coords::new(-2, 0),
            Direction::NW => Coords::new(-1, 2),
            Direction::NE => Coords::new(1, 2),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Coords {
    x: i32,
    y: i32,
}

impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Coords {
    fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }

    fn empty() -> Self {
        Coords::new(0, 0)
    }

    fn add_mut(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }

    fn add(&self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn get_neighbours(&self) -> Vec<Coords> {
        Direction::iter().map(|x| self.add(x.get_move())).collect()
    }

    fn get_with_neighbours(&self) -> Vec<Coords> {
        let mut res: Vec<Coords> = self.get_neighbours();
        res.push(*self);
        res
    }
}

fn paint_floor(input: &InputType) -> HashSet<Coords> {
    let mut black_tiles: HashSet<Coords> = HashSet::new();
    for dirs in input {
        let mut pos = Coords::empty();
        for d in dirs {
            pos.add_mut(d.get_move());
        }

        if black_tiles.contains(&pos) {
            black_tiles.remove(&pos);
        } else {
            black_tiles.insert(pos);
        }
    }

    black_tiles
}

fn part1(input: &InputType) -> String {
    paint_floor(input).len().to_string()
}

fn part2(input: &InputType) -> String {
    let mut black_tiles: HashSet<Coords> = paint_floor(input);

    let mut flips: HashSet<Coords> = HashSet::new();
    for _ in 0..100 {
        black_tiles
            .iter()
            .flat_map(|x| x.get_with_neighbours())
            .for_each(|x| {
                let is_black = black_tiles.contains(&x);
                let neighs = x
                    .get_neighbours()
                    .iter()
                    .filter(|&n| black_tiles.contains(n))
                    .count();

                if (is_black && (neighs == 0 || neighs > 2)) || (!is_black && neighs == 2) {
                    flips.insert(x);
                }
            });

        flips.drain().for_each(|x| {
            if black_tiles.contains(&x) {
                black_tiles.remove(&x);
            } else {
                black_tiles.insert(x);
            }
        });
    }

    black_tiles.len().to_string()
}

type InputType = Vec<Vec<Direction>>;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut res = Vec::new();

    for line in raw_input {
        let mut sub_res = Vec::new();
        let mut idx = 0;
        while idx < line.len() {
            if idx + 1 < line.len() {
                if let Ok(dir) = line[idx..idx + 2].parse::<Direction>() {
                    sub_res.push(dir);
                    idx += 2;
                    continue;
                }
            }

            if let Ok(dir) = line[idx..idx + 1].parse::<Direction>() {
                sub_res.push(dir);
                idx += 1;
            } else {
                panic!("whoopsie daisy");
            }
        }
        res.push(sub_res);
    }
    res
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
