use crate::common::Solution;
use std::fmt;

struct Map {
    data: Vec<bool>,
    xs: usize,
    ys: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "MAP({}, {}):", self.xs, self.ys)?;
        for y in 0..self.ys {
            for x in 0..self.xs {
                let val = self.get(x, y);
                write!(f, "{},", if val { '#' } else { ' ' })?;
            }
            writeln!(f, " ")?;
        }
        writeln!(f, "")
    }
}

impl Map {
    fn new(x: usize, y: usize) -> Map {
        Map {
            data: vec![false; x * y],
            xs: x,
            ys: y,
        }
    }

    fn build(&mut self, input: &Vec<String>) {
        for (ridx, row) in input.into_iter().enumerate() {
            for (cidx, value) in row.chars().enumerate() {
                self.set(cidx, ridx, value == '#')
            }
        }
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        let idx = x + (y * self.xs);
        self.data[idx] = val;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        let idx = x + (y * self.xs);
        self.data[idx]
    }
}

fn check_slope(data: &Map, xslope: usize, yslope: usize) -> usize {
    let mut tree_counter = 0;
    let mut ix = 0;
    let mut iy = 0;
    let x_limit = data.xs;
    let y_limit = data.ys;

    while iy < y_limit {
        if data.get(ix, iy) {
            tree_counter += 1;
        }
        ix += xslope;
        iy += yslope;

        if ix >= x_limit {
            ix -= x_limit
        }
    }
    tree_counter
}

fn check_slope2(data: &Map, xslope: usize, yslope: usize) -> usize {
    let x_limit = data.xs;
    let y_limit = data.ys;
    let slope_generator =
        itertools::iterate((0, 0), move |(x, y)| ((x + xslope) % x_limit, y + yslope));

    slope_generator
        .take_while(|&(_, y)| y < y_limit)
        .filter(|&(x, y)| data.get(x, y))
        .count()
}

fn part1(input: &Map) -> String {
    check_slope(input, 3, 1).to_string()
}

fn part2(input: &Map) -> String {
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|&(x, y)| check_slope2(input, x as usize, y as usize))
        .product::<usize>()
        .to_string()
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input: Vec<String> = raw_input.iter().map(|x| x.to_string()).collect();

    let mut data_map = Map::new(input[0].len(), input.len());
    data_map.build(&input);

    (part1(&data_map), part2(&data_map))
}
