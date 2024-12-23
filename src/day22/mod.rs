use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let secret_numbers = input.lines().map(|line| line.parse().unwrap()).collect::<Vec<i64>>();

    println!("PART 1: {}", part1(&secret_numbers));
    println!("PART 2: {}", part2(&secret_numbers));
}

fn part1(secret_numbers: &Vec<i64>) -> i64 {
    secret_numbers.iter()
        .map(|secret_number| *generate_n_secret_numbers(*secret_number, 2000).last().unwrap())
        .sum()
}

fn part2(secret_numbers: &Vec<i64>) -> i64 {
    let price_per_sequence_per_buyer = secret_numbers.iter()
        .map(|secret_number| {
            let numbers = generate_n_secret_numbers(*secret_number, 2000);
            let changes = numbers.iter().zip(numbers[1..].iter()).map(|(a, b)| (b % 10) - (a % 10)).collect::<Vec<i64>>();

            changes.into_iter().zip(numbers.into_iter().skip(1))
                .map_windows(|[(c1, _), (c2, _), (c3, _), (c4, value)]| {
                    (format!("{},{},{},{}", c1, c2, c3, c4), *value % 10)
                })
                .fold(HashMap::new(), |mut map, (sequence, price)| {
                    if !map.contains_key(&sequence) {
                        map.insert(sequence, price);
                    }

                    map
                })
        })
        .collect::<Vec<HashMap<String, i64>>>();

    let unique_sequences = price_per_sequence_per_buyer.iter()
        .flat_map(|prices_per_sequence| prices_per_sequence.keys().map(|sequence| sequence.clone()))
        .collect::<HashSet<String>>();

    unique_sequences.into_iter()
        .map(|sequence|
            price_per_sequence_per_buyer.iter()
                .map(|price_per_sequence| *price_per_sequence.get(&sequence).unwrap_or(&0))
                .sum()
        )
        .max()
        .unwrap()
}

fn generate_n_secret_numbers(mut secret_number: i64, n: usize) -> Vec<i64> {
    let mut numbers = vec![secret_number];

    for _ in 0..n {
        secret_number = generate_next_number(secret_number);
        numbers.push(secret_number);
    }

    numbers
}

fn generate_next_number(mut secret_number: i64) -> i64 {
    secret_number ^= secret_number * 64;
    secret_number %= 16777216;

    secret_number ^= secret_number / 32;
    secret_number %= 16777216;

    secret_number ^= secret_number * 2048;
    secret_number %= 16777216;

    secret_number
}
