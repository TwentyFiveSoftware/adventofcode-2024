use std::collections::HashMap;
use std::iter::zip;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    let (mut list1, mut list2) = parse_input_into_lists(input);

    list1.sort();
    list2.sort();

    zip(list1, list2)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (list1, list2) = parse_input_into_lists(input);

    let list2_number_counts = list2.into_iter()
        .fold(HashMap::new(), |mut map, number| {
            *map.entry(number).or_insert(0) += 1;
            map
        });

    list1
        .iter()
        .map(|number| number * list2_number_counts.get(number).unwrap_or(&0))
        .sum()
}

fn parse_input_into_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap_or_default());

            (numbers.next().unwrap_or_default(), numbers.next().unwrap_or_default())
        })
        .unzip()
}
