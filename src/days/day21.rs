use crate::{common::Solution, reparse};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PRODUCT_REGX: Regex = Regex::new(r"([\w ]+) \(contains ([\w, ]+)\)").unwrap();
}
type AllergenToProdMap = HashMap<String, HashSet<String>>;

fn resolve_mapping(allergen_to_prods: &mut AllergenToProdMap) -> HashMap<String, String> {
    let mut allerg_mapping: HashMap<String, String> = HashMap::new();

    while !allergen_to_prods.is_empty() {
        let singles: Vec<(String, String)> = allergen_to_prods
            .iter()
            .filter_map(|(al, prods)| {
                if prods.len() == 1 {
                    Some((al.to_string(), prods.iter().next().unwrap().to_string()))
                } else {
                    None
                }
            })
            .collect();

        singles.iter().for_each(|(al, prod)| {
            allerg_mapping.insert(al.to_string(), prod.to_string());

            allergen_to_prods.remove(al);
            allergen_to_prods.iter_mut().for_each(|(_, prods)| {
                prods.remove(prod);
            });
        });
    }

    allerg_mapping
}

fn find_common_prods(input: &InputType) -> AllergenToProdMap {
    input
        .all_allergens
        .iter()
        .map(|al| {
            let foods_with_al = input
                .data
                .iter()
                .filter(|&f| f.allergens.contains(al))
                .map(|f| &f.products)
                .collect::<Vec<&HashSet<String>>>();

            let common_prods: HashSet<String> = foods_with_al[0]
                .iter()
                .filter(|&prod| foods_with_al[1..].iter().all(|food| food.contains(prod)))
                .map(|prod| prod.to_string())
                .collect();
            (al.to_string(), common_prods)
        })
        .collect()
}

fn part1(input: &InputType) -> String {
    let mut allergen_to_prods = find_common_prods(input);
    let identified = resolve_mapping(&mut allergen_to_prods);
    let allergic_prods: HashSet<String> = identified.values().map(|x| x.to_string()).collect();

    input
        .data
        .iter()
        .flat_map(|f| f.products.iter())
        .filter(|&prod| !allergic_prods.contains(prod))
        .count()
        .to_string()
}

fn part2(input: &InputType) -> String {
    let mut allergen_to_prods = find_common_prods(input);
    let identified = resolve_mapping(&mut allergen_to_prods);
    let mut allergens_sorted: Vec<String> =
        input.all_allergens.iter().map(|x| x.to_string()).collect();
    allergens_sorted.sort_unstable();

    allergens_sorted
        .iter()
        .map(|key| identified.get(key).unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",")
        .to_string()
}

struct Food {
    products: HashSet<String>,
    allergens: HashSet<String>,
}

struct FoodList {
    data: Vec<Food>,
    all_allergens: HashSet<String>,
}

type InputType = FoodList;
fn parse_input(raw_input: &[String]) -> InputType {
    let mut all_allergens: HashSet<String> = HashSet::new();

    let data = raw_input
        .iter()
        .map(|x| {
            let (prod_str, allerg_str) = reparse!(x, PRODUCT_REGX, String, String).unwrap();
            let products: HashSet<String> =
                prod_str.split_whitespace().map(|x| x.to_string()).collect();
            let allergens: HashSet<String> =
                allerg_str.split(", ").map(|x| x.to_string()).collect();
            allergens.iter().for_each(|x| {
                all_allergens.insert(x.to_string());
            });

            Food {
                products,
                allergens,
            }
        })
        .collect();
    FoodList {
        data,
        all_allergens,
    }
}

pub fn solve(raw_input: &[String]) -> Solution {
    let input = parse_input(raw_input);

    use std::time::Instant;
    let now = Instant::now();
    let solution = (part1(&input), part2(&input));
    let elapsed = now.elapsed();
    (solution, elapsed)
}
