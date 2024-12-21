use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let memory_space = MemorySpace::from_str(input).unwrap();

    println!("PART 1: {}", part1(&memory_space));
    println!("PART 2: {}", part2(&memory_space));
}

fn part1(memory_space: &MemorySpace) -> u64 {
    memory_space.find_shortest_path(71, 71, 1024).unwrap()
}

fn part2(memory_space: &MemorySpace) -> String {
    for i in 1..memory_space.corrupted_coordinates.len() {
        if memory_space.find_shortest_path(71, 71, i).is_none() {
            let (x, y) = memory_space.corrupted_coordinates[i - 1];
            return format!("{},{}", x, y);
        }
    }

    "".to_string()
}

struct MemorySpace {
    corrupted_coordinates: Vec<(i32, i32)>,
}

impl MemorySpace {
    fn find_shortest_path(&self, width: i32, height: i32, number_of_corrupted_regions: usize) -> Option<u64> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            coordinate: (i32, i32),
            length: u64,
        }

        impl PartialOrd<Self> for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.length.cmp(&self.length).then(Ordering::Less)
            }
        }

        let corrupted_coordinates = self.corrupted_coordinates.iter().take(number_of_corrupted_regions)
            .collect::<HashSet<&(i32, i32)>>();

        let mut heap = BinaryHeap::new();
        heap.push(State { coordinate: (0, 0), length: 0 });

        let mut lengths = HashMap::new();
        lengths.insert((0, 0), 0);

        while let Some(current_state) = heap.pop() {
            let (x, y) = current_state.coordinate;

            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .filter(|(new_x, new_y)| *new_x >= 0 && *new_x < width && *new_y >= 0 && *new_y < height)
                .filter(|coords| !corrupted_coordinates.contains(coords))
                .map(|coords| State { coordinate: coords, length: current_state.length + 1 })
                .for_each(|state| {
                    if *lengths.get(&state.coordinate).unwrap_or(&u64::MAX) <= state.length {
                        return;
                    }

                    heap.push(state);

                    lengths.entry(state.coordinate)
                        .and_modify(|length| *length = state.length)
                        .or_insert(state.length);
                })
        }

        match lengths.get(&(width - 1, height - 1)) {
            Some(length) => Some(*length),
            None => None
        }
    }
}

impl FromStr for MemorySpace {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let corrupted_coordinates = s.lines()
            .filter_map(|line| {
                let [x, y] = line.split(",").collect::<Vec<&str>>()[..] else { return None; };
                Some((x.parse().unwrap(), y.parse().unwrap()))
            })
            .collect::<Vec<(i32, i32)>>();

        Ok(MemorySpace { corrupted_coordinates })
    }
}
