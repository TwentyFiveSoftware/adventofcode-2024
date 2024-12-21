use std::collections::{HashMap};
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let [raw_available_towels, raw_designs] = input.split("\n\n").collect::<Vec<&str>>()[..] else { return; };
    let available_towels = raw_available_towels.split(", ").map(|towel| towel.to_string()).collect::<Vec<String>>();
    let designs = raw_designs.split("\n").map(|design| design.to_string()).collect::<Vec<String>>();

    println!("PART 1: {}", part1(&available_towels, &designs));
    println!("PART 2: {}", part2(&available_towels, &designs));
}

fn part1(available_towels: &Vec<String>, designs: &Vec<String>) -> usize {
    let regex = Regex::new(&format!("^({})+$", available_towels.join("|"))).unwrap();

    designs.iter()
        .filter(|towel| regex.is_match(towel))
        .count()
}

fn part2(available_towels: &Vec<String>, designs: &Vec<String>) -> usize {
    fn number_of_possible_arrangements<'a>(design: &'a str, available_towels: &Vec<String>, cache: &mut HashMap<&'a str, usize>) -> usize {
        if design.is_empty() {
            return 1;
        }

        if cache.contains_key(design) {
            return *cache.get(design).unwrap();
        }

        let mut count = 0;

        for towel in available_towels {
            if design.starts_with(towel) {
                count += number_of_possible_arrangements(&design[towel.len()..], available_towels, cache);
            }
        }

        cache.insert(design, count);

        count
    }

    let mut cache = HashMap::new();

    designs.iter()
        .map(|design| number_of_possible_arrangements(design, available_towels, &mut cache))
        .sum()
}
