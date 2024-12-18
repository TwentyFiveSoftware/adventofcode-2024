use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let maze = Maze::from_str(input).unwrap();

    println!("PART 1: {}", part1(&maze));
    println!("PART 2: {}", part2(&maze));
}

fn part1(maze: &Maze) -> u64 {
    // this could also use the BFS algorithm from part2,
    // but I don't want to remove this version using the Dijkstra algorithm
    maze.find_path_with_best_score()
}

fn part2(maze: &Maze) -> usize {
    maze.find_all_paths_with_best_score()
}

struct Maze {
    empty_tiles: HashSet<(i32, i32)>,
    start_position: (i32, i32),
    end_position: (i32, i32),
}

impl Maze {
    fn find_path_with_best_score(&self) -> u64 {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            position: (i32, i32),
            direction: Direction,
            score: u64,
        }

        impl PartialOrd<Self> for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.score.cmp(&self.score).then_with(|| self.position.cmp(&other.position))
            }
        }

        let mut heap = BinaryHeap::new();
        heap.push(State { position: self.start_position, direction: Direction::East, score: 0 });

        let mut scores = HashMap::new();
        scores.insert((self.start_position, Direction::East), 0);

        while let Some(current_state) = heap.pop() {
            vec![
                State { position: current_state.direction.apply(current_state.position), direction: current_state.direction, score: current_state.score + 1 },
                State { position: current_state.position, direction: current_state.direction.turn_left(), score: current_state.score + 1000 },
                State { position: current_state.position, direction: current_state.direction.turn_right(), score: current_state.score + 1000 },
            ].into_iter()
                .filter(|state| self.empty_tiles.contains(&state.position))
                .for_each(|state| {
                    if *scores.get(&(state.position, state.direction)).unwrap_or(&u64::MAX) < state.score {
                        return;
                    }

                    heap.push(state);

                    scores.entry((state.position, state.direction))
                        .and_modify(|score| *score = state.score)
                        .or_insert(state.score);
                });
        }

        scores.into_iter()
            .filter_map(|((position, _), score)| {
                if position == self.end_position {
                    Some(score)
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn find_all_paths_with_best_score(&self) -> usize {
        struct State {
            position: (i32, i32),
            direction: Direction,
            score: u64,
            tiles: HashSet<(i32, i32)>,
        }

        let mut queue = VecDeque::new();
        queue.push_front(State {
            position: self.start_position,
            direction: Direction::East,
            score: 0,
            tiles: HashSet::from_iter(vec![self.start_position].into_iter()),
        });

        let mut scores = HashMap::new();
        scores.insert((self.start_position, Direction::East), 0);

        let mut tiles_per_score = HashMap::new();

        while let Some(current_state) = queue.pop_front() {
            if current_state.position == self.end_position {
                tiles_per_score.entry(current_state.score)
                    .and_modify(|tiles: &mut HashSet<(i32, i32)>| current_state.tiles.iter().for_each(|tile| {
                        tiles.insert(*tile);
                    }))
                    .or_insert(current_state.tiles);
                continue;
            }

            let mut new_states = vec![
                State {
                    position: current_state.position,
                    direction: current_state.direction.turn_left(),
                    score: current_state.score + 1000,
                    tiles: current_state.tiles.clone(),
                },
                State {
                    position: current_state.position,
                    direction: current_state.direction.turn_right(),
                    score: current_state.score + 1000,
                    tiles: current_state.tiles.clone(),
                },
            ];

            let new_position = current_state.direction.apply(current_state.position);
            if self.empty_tiles.contains(&new_position) {
                let mut tiles_clone = current_state.tiles.clone();
                tiles_clone.insert(new_position);

                new_states.push(State {
                    position: new_position,
                    direction: current_state.direction,
                    score: current_state.score + 1,
                    tiles: tiles_clone,
                });
            }

            new_states.into_iter().for_each(|state| {
                if *scores.get(&(state.position, state.direction)).unwrap_or(&u64::MAX) < state.score {
                    return;
                }

                scores.entry((state.position, state.direction))
                    .and_modify(|score| *score = state.score)
                    .or_insert(state.score);

                queue.push_back(state);
            });
        }

        let best_score = tiles_per_score.keys().min().unwrap();
        tiles_per_score.get(best_score).unwrap().len()
    }
}

impl FromStr for Maze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut empty_tiles: HashSet<(i32, i32)> = HashSet::new();
        let mut start_position = (0, 0);
        let mut end_position = (0, 0);

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
                        end_position = position;
                        empty_tiles.insert(position);
                    }
                    _ => {}
                }
            }));

        Ok(Maze { empty_tiles, start_position, end_position })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}
