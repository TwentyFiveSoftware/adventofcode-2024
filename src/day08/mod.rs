use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let map = Map::from_str(input).unwrap();

    println!("PART 1: {}", part1(&map));
    println!("PART 2: {}", part2(&map));
}

fn part1(map: &Map) -> usize {
    map.get_combinations_of_antennas_with_same_frequency()
        .flat_map(|(antenna, other_antenna)| {
            let dx = other_antenna.x - antenna.x;
            let dy = other_antenna.y - antenna.y;

            vec![
                (antenna.x - dx, antenna.y - dy),
                (other_antenna.x + dx, other_antenna.y + dy),
            ]
        })
        .filter(|(x, y)| map.is_on_map(*x, *y))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

fn part2(map: &Map) -> usize {
    map.get_combinations_of_antennas_with_same_frequency()
        .flat_map(|(antenna, other_antenna)| {
            let dx = other_antenna.x - antenna.x;
            let dy = other_antenna.y - antenna.y;

            let mut locations: HashSet<(i32, i32)> = HashSet::new();

            {
                let mut x = antenna.x;
                let mut y = antenna.y;

                while map.is_on_map(x, y) {
                    locations.insert((x, y));
                    x += dx;
                    y += dy;
                }
            }

            {
                let mut x = antenna.x;
                let mut y = antenna.y;

                while map.is_on_map(x, y) {
                    locations.insert((x, y));
                    x -= dx;
                    y -= dy;
                }
            }

            locations
        })
        .filter(|(x, y)| map.is_on_map(*x, *y))
        .collect::<HashSet<(i32, i32)>>()
        .len()
}

struct Map {
    width: i32,
    height: i32,
    antennas_per_frequency: HashMap<char, Vec<Antenna>>,
}

impl Map {
    fn is_on_map(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    fn get_combinations_of_antennas_with_same_frequency(&self) -> impl Iterator<Item=(&Antenna, &Antenna)> {
        self.antennas_per_frequency.iter()
            .flat_map(|(_, antennas)| {
                let mut combinations = vec![];

                for i in 0..antennas.len() {
                    for j in (i + 1)..antennas.len() {
                        let (antenna, other_antenna) = (&antennas[i], &antennas[j]);
                        combinations.push((antenna, other_antenna));
                    }
                }

                combinations
            })
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antennas_per_frequency = s.lines().enumerate()
            .flat_map(move |(y, row)| row.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| Antenna { x: x as i32, y: y as i32, frequency: c }))
            .fold(HashMap::new(), |mut antennas, antenna| {
                antennas.entry(antenna.frequency).or_insert(vec![]).push(antenna);
                antennas
            });

        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;

        Ok(Map { width, height, antennas_per_frequency })
    }
}

struct Antenna {
    x: i32,
    y: i32,
    frequency: char,
}
