use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let maze = Maze::from_str(input).unwrap();

    println!("PART 1: {}", part1(&maze));
    println!("PART 2: {}", part2(&maze));
}

fn part1(maze: &Maze) -> usize {
    maze.find_cheat_count(2, 100)
}

fn part2(maze: &Maze) -> usize {
    maze.find_cheat_count(20, 100)
}

struct Maze {
    empty_tiles: HashSet<(i32, i32)>,
    start_position: (i32, i32),
}

impl Maze {
    fn find_cheat_count(&self, max_cheat_length: i32, min_time_save: i32) -> usize {
        let base_time_to_each_tile = self.find_shortest_path_to_each_tile();

        let mut cheat_count = 0;

        for (position_on_path, length_to_position) in base_time_to_each_tile.clone() {
            cheat_count += self.find_all_cheat_end_positions_for_start_position(position_on_path, max_cheat_length).into_iter()
                .map(|(end_position, cheat_length)| *base_time_to_each_tile.get(&end_position).unwrap() - (length_to_position + cheat_length))
                .filter(|time_save| *time_save >= min_time_save)
                .count()
        }

        cheat_count
    }

    fn find_shortest_path_to_each_tile(&self) -> HashMap<(i32, i32), i32> {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            position: (i32, i32),
            length: i32,
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

        let mut heap = BinaryHeap::new();
        heap.push(State { position: self.start_position, length: 0 });

        let mut lengths = HashMap::new();
        lengths.insert(self.start_position, 0);

        while let Some(current_state) = heap.pop() {
            let (x, y) = current_state.position;

            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .filter(|position| self.empty_tiles.contains(position))
                .map(|position| State { position, length: current_state.length + 1 })
                .for_each(|state| {
                    if *lengths.get(&state.position).unwrap_or(&i32::MAX) <= state.length {
                        return;
                    }

                    heap.push(state);

                    lengths.entry(state.position)
                        .and_modify(|length| *length = state.length)
                        .or_insert(state.length);
                })
        }

        lengths
    }

    fn find_all_cheat_end_positions_for_start_position(&self, (start_x, start_y): (i32, i32), max_length: i32) -> HashMap<(i32, i32), i32> {
        let mut end_positions = HashMap::new();

        for x in (start_x - max_length)..(start_x + max_length + 1) {
            for y in (start_y - max_length)..(start_y + max_length + 1) {
                if !self.empty_tiles.contains(&(x, y)) {
                    continue;
                }

                let distance = (start_x - x).abs() + (start_y - y).abs();
                if distance <= max_length {
                    end_positions.insert((x, y), distance);
                }
            }
        }

        end_positions
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut empty_tiles: HashSet<(i32, i32)> = HashSet::new();
        let mut start_position = (0, 0);

        s.lines().enumerate()
            .for_each(|(y, row)| row.char_indices().for_each(|(x, c)| {
                let position = (x as i32, y as i32);

                match c {
                    '.' => {
                        empty_tiles.insert(position);
                    }
                    'S' => {
                        start_position = position;
                        empty_tiles.insert(position);
                    }
                    'E' => {
                        empty_tiles.insert(position);
                    }
                    _ => {}
                }
            }));

        Ok(Maze { empty_tiles, start_position })
    }
}
