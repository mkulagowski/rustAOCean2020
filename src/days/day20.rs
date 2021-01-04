use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

lazy_static! {
    static ref TITLE_REGX: Regex = Regex::new(r"Tile (\d+):").unwrap();
    static ref SNAKE_ALIGNMENT: [Vec<usize>; 3] = [
        vec![1, 4, 7, 10, 13, 16],
        vec![0, 5, 6, 11, 12, 17, 18, 19],
        vec![18]
    ];
}

static SNAKE_HEIGHT: usize = 3;
static SNAKE_LENGTH: usize = 20;

#[derive(Debug, Clone)]
struct Tile {
    id: u32,
    data: Vec<Vec<bool>>,
    borders: Vec<u16>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            for val in row {
                write!(f, "{} ", if *val { '#' } else { '.' })?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")
    }
}

impl Tile {
    fn get_borders(&self) -> impl Iterator<Item = &u16> {
        self.borders.iter()
    }

    fn get_flipped_borders(&self) -> Vec<u16> {
        self.borders.iter().map(|b| Tile::flip_border(*b)).collect()
    }

    fn rotr_data(&mut self) {
        let mut new_data: Vec<Vec<bool>> = vec![vec![false; self.data.len()]; self.data.len()];

        for (iy, row) in self.data.iter().rev().enumerate() {
            for (ix, val) in row.iter().enumerate() {
                if *val {
                    new_data[ix][iy] = true;
                }
            }
        }
        self.data = new_data;
    }

    fn rotr_borders(&mut self) {
        self.borders.rotate_right(1);
    }

    fn rotr(&mut self) {
        self.rotr_borders();
        self.rotr_data();
    }

    fn flip_borders(&mut self) {
        let new_borders = self
            .borders
            .drain(..)
            .map(|x| Tile::flip_border(x))
            .collect();
        self.borders = new_borders;
        let top = self.top();
        let bottom = self.bottom();
        self.borders[0] = bottom;
        self.borders[2] = top;
    }

    fn flip_data(&mut self) {
        let mut new_data: Vec<Vec<bool>> = vec![vec![false; self.data.len()]; self.data.len()];
        for (iy, row) in self.data.iter().rev().enumerate() {
            for (ix, val) in row.iter().enumerate() {
                if *val {
                    new_data[iy][ix] = true;
                }
            }
        }
        self.data = new_data;
    }

    fn flip(&mut self) {
        self.flip_data();
        self.flip_borders();
    }

    fn top(&self) -> u16 {
        self.borders[0]
    }

    fn left(&self) -> u16 {
        self.borders[3]
    }

    fn right(&self) -> u16 {
        self.borders[1]
    }

    fn bottom(&self) -> u16 {
        self.borders[2]
    }

    fn check_snake(&self, x: usize, y: usize) -> bool {
        SNAKE_ALIGNMENT
            .iter()
            .enumerate()
            .all(|(yy, xxs)| xxs.iter().all(|xx| self.data[y - yy][x + xx]))
    }

    fn kill_snake(&mut self, x: usize, y: usize) {
        SNAKE_ALIGNMENT
            .iter()
            .enumerate()
            .for_each(|(yy, xxs)| xxs.iter().for_each(|xx| self.data[y - yy][x + xx] = false));
    }

    fn find_and_destroy_snakes(&mut self) -> bool {
        let y_max = self.data.len();
        let x_max = self.data[0].len();

        let coords: Vec<(usize, usize)> = (SNAKE_HEIGHT - 1..y_max)
            .into_iter()
            .flat_map(|y| {
                (0..x_max - (SNAKE_LENGTH - 1))
                    .into_iter()
                    .filter(|x| self.check_snake(x.clone(), y))
                    .map(|x| (x, y))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect();

        if coords.is_empty() {
            false
        } else {
            coords.into_iter().for_each(|(x, y)| self.kill_snake(x, y));
            true
        }
    }

    fn obliterate_all_snakes(&mut self) {
        let mut rot_counter = 0;
        while !self.find_and_destroy_snakes() {
            self.rotr_data();
            rot_counter += 1;

            if rot_counter == 4 {
                self.flip_data();
            } else if rot_counter > 8 {
                break;
            }
        }
    }

    fn flip_border(val: u16) -> u16 {
        val.reverse_bits() >> 6
    }
}

fn count_borders(input: &InputType) -> HashMap<u16, HashSet<u32>> {
    let mut border_counter: HashMap<u16, HashSet<u32>> = HashMap::new();
    input.iter().for_each(|t| {
        t.get_borders()
            .chain(t.get_flipped_borders().iter())
            .for_each(|x| {
                border_counter.entry(*x).or_default().insert(t.id);
            })
    });
    border_counter
}

fn map_neighbours(border_counter: &HashMap<u16, HashSet<u32>>) -> HashMap<u32, HashSet<u32>> {
    let mut neigh_counter: HashMap<u32, HashSet<u32>> = HashMap::new();
    border_counter.iter().for_each(|(_, tiles)| {
        tiles.iter().for_each(|&id| {
            tiles.iter().filter(|&&i| i != id).for_each(|&i| {
                neigh_counter.entry(id).or_default().insert(i);
            })
        })
    });
    neigh_counter
}

fn _print_image(image: &Vec<Vec<Tile>>) {
    for row in image {
        for y in 0..row[0].data.len() {
            for t in row {
                for val in &t.data[y] {
                    print!("{} ", if *val { '#' } else { '.' });
                }
                print!("  ");
            }
            println!("");
        }
        println!("");
    }
}

fn align_first_tile(tile: &mut Tile, wall_to_ids: &HashMap<u16, HashSet<u32>>) {
    loop {
        let right = tile.right();
        let bottom = tile.bottom();
        let is_right_match = wall_to_ids.get(&right).unwrap().len() == 2;
        let is_bottom_match = wall_to_ids.get(&bottom).unwrap().len() == 2;

        if is_right_match && is_bottom_match {
            break;
        }
        tile.rotr();
    }
}

fn match_tile<P>(
    id: u32,
    wall: u16,
    wall_to_ids: &HashMap<u16, HashSet<u32>>,
    tiles_left: &mut Vec<Tile>,
    mut map_to_wall: P,
) -> Tile
where
    P: FnMut(&Tile) -> u16,
{
    let tile_id = wall_to_ids
        .get(&wall)
        .unwrap()
        .iter()
        .find(|&x| x != &id)
        .unwrap();
    let (idx, _) = tiles_left
        .iter()
        .enumerate()
        .find(|(_, x)| &x.id == tile_id)
        .unwrap();
    let mut next = tiles_left.remove(idx);
    if next.get_borders().all(|&x| x != wall) {
        next.flip();
    }
    while map_to_wall(&next) != wall {
        next.rotr();
    }
    next
}

fn merge_tiles(tiles: Vec<Vec<Tile>>) -> Tile {
    let mut data: Vec<Vec<bool>> = Vec::new();
    for row in tiles {
        for y in 1..row[0].data.len() - 1 {
            let mut new_row = Vec::new();
            for t in &row {
                let trimmed_row: Vec<&bool> =
                    t.data[y].iter().skip(1).take(t.data[0].len() - 2).collect();
                for val in trimmed_row {
                    new_row.push(*val);
                }
            }
            data.push(new_row);
        }
    }
    Tile {
        id: 0,
        data,
        borders: Vec::new(),
    }
}

fn part1(input: &InputType) -> String {
    let wall_to_ids = count_borders(input);
    map_neighbours(&wall_to_ids)
        .iter()
        .filter(|(_, neighs)| neighs.len() == 2)
        .map(|(&id, _)| id as u64)
        .product::<u64>()
        .to_string()
}

fn part2(input: &InputType) -> String {
    let wall_to_ids = count_borders(input);
    let tile_to_ids = map_neighbours(&wall_to_ids);
    let mut corner = input
        .iter()
        .find(|x| tile_to_ids.get(&x.id).unwrap().len() == 2)
        .unwrap()
        .clone();

    let mut tiles_left: Vec<Tile> = input
        .into_iter()
        .cloned()
        .filter(|x| x.id != corner.id)
        .collect();
    align_first_tile(&mut corner, &wall_to_ids);

    let mut image: Vec<Vec<Tile>> = vec![vec![corner.to_owned()]];
    let mut curr_id: u32;
    let mut curr_wall: u16;

    loop {
        let curr_tile = image.last().unwrap().last().unwrap();
        curr_id = curr_tile.id;
        curr_wall = Tile::flip_border(curr_tile.right());
        while wall_to_ids.get(&curr_wall).unwrap().len() > 1 {
            let next = match_tile(curr_id, curr_wall, &wall_to_ids, &mut tiles_left, |x| {
                x.left()
            });
            curr_id = next.id;
            curr_wall = Tile::flip_border(next.right());
            image.last_mut().unwrap().push(next);
        }

        if tiles_left.is_empty() {
            break;
        }

        let curr_tile = image.last().unwrap().first().unwrap();
        curr_id = curr_tile.id;
        curr_wall = Tile::flip_border(curr_tile.bottom());
        let next = match_tile(curr_id, curr_wall, &wall_to_ids, &mut tiles_left, |x| {
            x.top()
        });
        image.push(vec![next]);
    }

    let mut merged_image = merge_tiles(image);
    merged_image.obliterate_all_snakes();
    merged_image
        .data
        .into_iter()
        .flatten()
        .filter(|x| *x)
        .count()
        .to_string()
}

type InputType = Vec<Tile>;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut iter = raw_input.iter();
    let mut tiles = Vec::new();
    while let Some(line) = iter.next() {
        if !line.is_empty() {
            let id = reparse!(line, TITLE_REGX, u32).unwrap();
            let data: Vec<Vec<bool>> = iter
                .by_ref()
                .take(10)
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect();

            let mut unparsed_borders = Vec::new();
            unparsed_borders.push(data[0].clone());
            unparsed_borders.push(data.iter().map(|x| x[9]).collect());
            unparsed_borders.push(data[9].iter().rev().cloned().collect());
            unparsed_borders.push(data.iter().rev().map(|x| x[0]).collect());

            let borders = unparsed_borders
                .iter()
                .map(|x| {
                    let mut res = 0u16;
                    for i in x.iter() {
                        res <<= 1;
                        if *i {
                            res |= 1;
                        }
                    }
                    res
                })
                .collect();

            tiles.push(Tile { id, data, borders });
        }
    }
    tiles
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
