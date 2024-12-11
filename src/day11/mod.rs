use std::collections::{HashMap};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let stone_arrangement = StoneArrangement::from_str(input).unwrap();

    println!("PART 1: {}", part1(&stone_arrangement));
    println!("PART 2: {}", part2(&stone_arrangement));
}

fn part1(stone_arrangement: &StoneArrangement) -> usize {
    stone_arrangement.transformed_n_times(25).stone_count()
}

fn part2(stone_arrangement: &StoneArrangement) -> usize {
    stone_arrangement.transformed_n_times(75).stone_count()
}

#[derive(Clone)]
struct StoneArrangement {
    stones: HashMap<Stone, usize>,
}

impl StoneArrangement {
    fn transformed_n_times(&self, n: usize) -> StoneArrangement {
        let mut arrangement = self.clone();

        for _ in 0..n {
            arrangement.transform();
        }

        arrangement
    }

    fn transform(&mut self) {
        self.stones = self.stones.iter()
            .fold(HashMap::new(), |mut stones, (stone, amount)| {
                stone.transformed().into_iter().for_each(|new_stone| {
                    *stones.entry(new_stone).or_insert(0) += amount;
                });

                stones
            });
    }

    fn stone_count(&self) -> usize {
        self.stones.values().sum()
    }
}

impl FromStr for StoneArrangement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s.split_whitespace()
            .map(|s| Stone::from_str(s).unwrap())
            .fold(HashMap::new(), |mut stones, stone| {
                *stones.entry(stone).or_insert(0) += 1;
                stones
            });

        Ok(StoneArrangement { stones })
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Stone {
    number: String,
}

impl Stone {
    fn transformed(&self) -> Vec<Stone> {
        if self.number.len() == 1 && self.number == "0" {
            return vec![Stone { number: "1".to_string() }];
        }

        if self.number.len() % 2 == 0 {
            return vec![
                Stone::from(self.number[..self.number.len() / 2].to_string()),
                Stone::from(self.number[self.number.len() / 2..].to_string()),
            ];
        }

        vec![Stone::from((self.as_number() * 2024).to_string())]
    }

    fn as_number(&self) -> u64 {
        self.number.parse().unwrap()
    }
}

impl From<String> for Stone {
    fn from(value: String) -> Self {
        let number = value.trim_start_matches('0').to_owned();

        Stone {
            number: if number.len() == 0 { "0".to_string() } else { number }
        }
    }
}

impl FromStr for Stone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Stone { number: s.to_owned() })
    }
}
