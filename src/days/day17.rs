use crate::common::Solution;
use crate::points::{Point, Point3, Point4};
use std::collections::HashSet;
use std::hash::Hash;

enum State {
    Active,
    Inactive,
}

impl State {
    fn change(&self, active_neights: usize) -> Self {
        match self {
            Self::Active => Self::from_bool(active_neights == 2 || active_neights == 3),
            Self::Inactive => Self::from_bool(active_neights == 3),
        }
    }

    fn from_bool(state: bool) -> Self {
        match state {
            true => Self::Active,
            false => Self::Inactive,
        }
    }
}

struct CycleState<T>
where
    T: Point + Copy + Eq + Hash,
{
    active_points: HashSet<T>,
    points_to_check: HashSet<T>,
}

fn step_cycle<T>(state: &mut CycleState<T>)
where
    T: Point + Copy + Eq + Hash,
{
    let mut new_points: HashSet<T> = HashSet::new();
    let mut new_act_points: HashSet<T> = HashSet::new();
    let mut new_dact_points: HashSet<T> = HashSet::new();

    state.points_to_check.iter().for_each(|curr_point| {
        let point_state = State::from_bool(state.active_points.contains(curr_point));
        let neighs: Vec<T> = curr_point.get_neighbours().into_iter().collect();
        let active_neigh_num = neighs
            .iter()
            .filter(|&neighbour| state.active_points.contains(neighbour))
            .count();

        match point_state {
            State::Active => match point_state.change(active_neigh_num) {
                State::Active => {
                    neighs.into_iter().for_each(|neighbour| {
                        new_points.insert(neighbour);
                    });
                    new_points.insert(*curr_point);
                }
                State::Inactive => {
                    new_dact_points.insert(*curr_point);
                }
            },
            State::Inactive => match point_state.change(active_neigh_num) {
                State::Active => {
                    neighs.into_iter().for_each(|neighbour| {
                        new_points.insert(neighbour);
                    });
                    new_points.insert(*curr_point);
                    new_act_points.insert(*curr_point);
                }
                State::Inactive => (),
            },
        }
    });
    state.points_to_check = new_points.drain().collect();
    new_dact_points.drain().for_each(|p| {
        state.active_points.remove(&p);
    });
    new_act_points.drain().for_each(|p| {
        state.active_points.insert(p);
    });
}

fn infinite_conway<T>(input: &InputType, cycles: usize) -> usize
where
    T: Point + Copy + Eq + Hash,
{
    let active_points: HashSet<T> = input.iter().copied().map(T::new).collect();
    let points_to_check: HashSet<T> = active_points
        .iter()
        .flat_map(|p| p.get_neighbours())
        .chain(active_points.iter().copied())
        .collect();
    let mut conway_state: CycleState<T> = CycleState {
        active_points,
        points_to_check,
    };

    for _ in 0..cycles {
        step_cycle(&mut conway_state);
    }

    conway_state.active_points.len()
}

fn part1(input: &InputType) -> String {
    infinite_conway::<Point3>(input, 6).to_string()
}

fn part2(input: &InputType) -> String {
    infinite_conway::<Point4>(input, 6).to_string()
}

type InputType = Vec<(i32, i32, i32, i32)>;
fn parse_input(raw_input: &[String]) -> InputType {
    raw_input
        .into_iter()
        .enumerate()
        .flat_map(|(ridx, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, value)| *value == '#')
                .map(|(cidx, _)| (cidx as i32, ridx as i32, 0, 0))
                .collect::<Vec<(i32, i32, i32, i32)>>()
        })
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
