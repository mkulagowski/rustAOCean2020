use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fmt};

lazy_static! {
    static ref TICKET_FIELD: Regex = Regex::new(r"([a-z ]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

type Ticket = Vec<u32>;
type TicketFields = HashMap<String, Ranges>;

#[derive(Copy, Clone)]
struct Ranges {
    from1: u32,
    to1: u32,
    from2: u32,
    to2: u32,
}

impl fmt::Display for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}-{} || {}-{})",
            self.from1, self.to1, self.from2, self.to2
        )
    }
}

struct TicketData {
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
    fields: TicketFields,
}

fn check_range(x: u32, range: Ranges) -> bool {
    (x >= range.from1 && x <= range.to1) || (x >= range.from2 && x <= range.to2)
}

// Drains field_counter!
fn resolve_into_field_mapping(
    field_counter: &mut HashMap<String, Vec<bool>>,
) -> HashMap<String, usize> {
    let mut field_mapping: HashMap<String, usize> = HashMap::new();
    while !field_counter.is_empty() {
        let good_fields: Vec<(String, usize)> = field_counter
            .iter()
            .filter_map(|(x, y)| {
                let goods: Vec<(usize, bool)> = y
                    .iter()
                    .enumerate()
                    .map(|(i, &vv)| (i, vv))
                    .filter(|(_, vv)| *vv)
                    .collect();
                if goods.len() == 1 {
                    Some((x.to_owned(), goods[0].0))
                } else {
                    None
                }
            })
            .collect();
        good_fields.iter().for_each(|(k, v)| {
            let field = field_mapping.entry(k.to_owned()).or_default();
            *field = *v;
            field_counter.remove(k);
            field_counter.iter_mut().for_each(|(_, x)| x[*v] = false);
        });
    }

    field_mapping
}

fn part1(input: &InputType) -> String {
    input
        .tickets
        .iter()
        .flat_map(|x| {
            x.iter()
                .filter_map(|&y| {
                    if !input.fields.iter().any(|(_, &range)| check_range(y, range)) {
                        return Some(y);
                    }
                    None
                })
                .collect::<Vec<u32>>()
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &InputType) -> String {
    let mut counter: HashMap<String, Vec<bool>> = input
        .fields
        .keys()
        .map(|k| (k.to_owned(), vec![true; input.tickets[0].len()]))
        .collect();

    input
        .tickets
        .iter()
        .filter(|&x| {
            let res = x.iter().all(|&y| {
                input.fields.iter().any(|(_, &range)| {
                    let ress = check_range(y, range);
                    ress
                })
            });
            res
        })
        .for_each(|ticket_vals| {
            ticket_vals.iter().enumerate().for_each(|(idx, &y)| {
                for (name, range) in input.fields.iter() {
                    if !check_range(y, *range) {
                        let ctr = counter.entry(name.to_owned()).or_default();
                        ctr[idx] = false;
                    }
                }
            })
        });

    let field_mapping = resolve_into_field_mapping(&mut counter);

    input
        .fields
        .keys()
        .filter(|&x| x.starts_with("departure"))
        .map(|x| input.my_ticket[*field_mapping.get(x).unwrap()])
        .product::<u32>()
        .to_string()
}

type InputType = TicketData;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut data = TicketData {
        my_ticket: Vec::new(),
        tickets: Vec::new(),
        fields: HashMap::new(),
    };
    let mut iter = raw_input.iter();
    while let Some(line) = iter.next() {
        if line.starts_with("your ticket:") {
            break;
        }
        if let Ok((name, from1, to1, from2, to2)) =
            reparse!(line, TICKET_FIELD, String, u32, u32, u32, u32)
        {
            data.fields.insert(
                name,
                Ranges {
                    from1,
                    to1,
                    from2,
                    to2,
                },
            );
        }
    }

    data.my_ticket = iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    while let Some(line) = iter.next() {
        if line.starts_with("nearby tickets:") {
            break;
        }
    }

    while let Some(line) = iter.next() {
        data.tickets
            .push(line.split(",").map(|x| x.parse().unwrap()).collect());
    }

    data
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
