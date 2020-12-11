use crate::common::Solution;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Clone)]
struct Map {
    data: Vec<Seat>,
    xs: usize,
    ys: usize,
    neigh_map: HashMap<(usize, usize), Vec<(usize, usize)>>,
    occupied_limit: usize,
}

#[derive(Copy, Clone, PartialEq, Hash, Eq, Debug)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
            Seat::Floor => write!(f, "_"),
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MAP({}, {}):", self.xs, self.ys)?;
        for y in 0..self.ys {
            for x in 0..self.xs {
                let val = self.get(x, y);
                write!(f, "{},", val)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

type CheckSeatFunctor = for<'r> fn(&'r Map, usize, usize, i32, i32) -> Option<(usize, usize)>;
impl Map {
    fn new(x: usize, y: usize) -> Map {
        Map {
            data: vec![Seat::Floor; x * y],
            xs: x,
            ys: y,
            neigh_map: HashMap::new(),
            occupied_limit: 0,
        }
    }

    fn build(&mut self, input: &Vec<String>) {
        for (ridx, row) in input.into_iter().enumerate() {
            for (cidx, value) in row.chars().enumerate() {
                if value == 'L' {
                    self.set(cidx, ridx, Seat::Empty);
                }
            }
        }
    }

    fn set(&mut self, x: usize, y: usize, val: Seat) {
        let idx = x + (y * self.xs);
        self.data[idx] = val;
    }

    fn get(&self, x: usize, y: usize) -> Seat {
        let idx = x + (y * self.xs);
        self.data[idx]
    }

    fn check_seat(&self, x: usize, y: usize, x_d: i32, y_d: i32) -> Option<(usize, usize)> {
        let xx = (x_d + (x as i32)) as usize;
        let yy = (y_d + (y as i32)) as usize;

        if xx >= self.xs || yy >= self.ys {
            return None;
        }
        let idx = xx + (yy * self.xs);
        match self.data[idx] {
            Seat::Floor => None,
            _ => Some((xx, yy)),
        }
    }

    fn check_seat_ex(&self, x: usize, y: usize, x_d: i32, y_d: i32) -> Option<(usize, usize)> {
        let mut xx = x as i32;
        let mut yy = y as i32;
        loop {
            xx += x_d;
            yy += y_d;

            if xx as usize >= self.xs || yy as usize >= self.ys {
                break None;
            }
            let idx = (xx + (yy * self.xs as i32)) as usize;
            match self.data[idx] {
                Seat::Floor => continue,
                _ => break Some((xx as usize, yy as usize)),
            }
        }
    }

    fn get_neighbours(&self, x: usize, y: usize, is_extended: bool) -> Vec<(usize, usize)> {
        let check_seat_func: CheckSeatFunctor = if is_extended {
            Map::check_seat_ex
        } else {
            Map::check_seat
        };

        [-1, 0, 1]
            .iter()
            .map(|&xx| {
                [-1, 0, 1]
                    .iter()
                    .filter(|&&yy| !(yy == 0 && xx == 0))
                    .filter_map(|&yy| check_seat_func(self, x, y, xx, yy))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }

    fn calc_neighs(&mut self, is_extended: bool) {
        for y in 0..self.ys {
            for x in 0..self.xs {
                self.neigh_map
                    .insert((x, y), self.get_neighbours(x, y, is_extended));
            }
        }
    }

    fn calc_change(&self, x: usize, y: usize) -> Option<Seat> {
        let curr = self.get(x, y);
        if let Seat::Floor = curr {
            return None;
        }

        let occupied = self
            .neigh_map
            .get(&(x, y))
            .unwrap()
            .into_iter()
            .map(|(xx, yy)| self.get(*xx, *yy))
            .filter(|&n| n == Seat::Occupied)
            .count();

        match curr {
            Seat::Empty => {
                if occupied == 0 {
                    return Some(Seat::Occupied);
                }
            }
            Seat::Occupied => {
                if occupied >= self.occupied_limit {
                    return Some(Seat::Empty);
                }
            }
            _ => {}
        }

        None
    }

    fn find_equilibrium(&mut self) -> usize {
        let mut changes: HashSet<(usize, usize, Seat)> = HashSet::new();
        loop {
            for y in 0..self.ys {
                for x in 0..self.xs {
                    if let Some(change) = self.calc_change(x, y) {
                        changes.insert((x, y, change));
                    }
                }
            }
            if changes.is_empty() {
                break;
            }

            changes
                .iter()
                .for_each(|(x, y, val)| self.set(*x, *y, *val));
            changes.clear();
        }

        self.data.iter().filter(|&&s| s == Seat::Occupied).count()
    }
}

fn part1(input: &Map) -> String {
    let mut board = input.clone();
    board.occupied_limit = 4;
    board.calc_neighs(false);

    board.find_equilibrium().to_string()
}

fn part2(input: &Map) -> String {
    let mut board = input.clone();
    board.occupied_limit = 5;
    board.calc_neighs(true);

    board.find_equilibrium().to_string()
}

fn parse_input(raw_input: &[String]) -> Map {
    let input: Vec<String> = raw_input.iter().map(|x| x.parse().unwrap()).collect();

    let mut data_map = Map::new(input[0].len(), input.len());
    data_map.build(&input);
    data_map
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
