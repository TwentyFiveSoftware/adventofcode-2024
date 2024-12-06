use std::collections::HashSet;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let grid = Grid::from_str(input).unwrap();

    println!("PART 1: {}", part1(&grid));
    println!("PART 2: {}", part2(&grid));
}

fn part1(grid: &Grid) -> usize {
    grid.simulate_route_length().unwrap()
}

fn part2(grid: &Grid) -> usize {
    (0..grid.width)
        .flat_map(move |x| (0..grid.height).map(move |y| (x, y)))
        .filter(|(x, y)| !(*x == grid.starting_position.0 && *y == grid.starting_position.1))
        .map(|new_obstacle_position| grid.with_obstacle_at(new_obstacle_position))
        .filter(|new_grid| new_grid.simulate_route_length().is_none())
        .count()
}

#[derive(Debug)]
struct Grid {
    obstacles: HashSet<(i32, i32)>,
    starting_position: (i32, i32),
    width: i32,
    height: i32,
}

impl Grid {
    fn simulate_route_length(&self) -> Option<usize> {
        let (mut x, mut y) = self.starting_position;
        let mut direction = Direction::Up;

        let mut visited_coordinates = HashSet::new();
        let mut visited_states = HashSet::new();

        while x >= 0 && x < self.width && y >= 0 && y < self.height {
            if visited_states.contains(&(x, y, direction)) {
                return None; // loop detected
            }

            visited_coordinates.insert((x, y));
            visited_states.insert((x, y, direction));

            let next_position = direction.move_in_direction((x, y));
            if self.obstacles.contains(&next_position) {
                direction = direction.turn_right();
                continue;
            }

            (x, y) = next_position;
        }

        Some(visited_coordinates.len())
    }

    fn with_obstacle_at(&self, new_obstacle_position: (i32, i32)) -> Grid {
        let mut new_obstacles = self.obstacles.clone();
        new_obstacles.insert(new_obstacle_position);

        Grid {
            obstacles: new_obstacles,
            starting_position: self.starting_position,
            width: self.width,
            height: self.height,
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let obstacles = s.lines().enumerate()
            .flat_map(|(y, row)| row.char_indices()
                .filter_map(move |(x, char)| match char {
                    '#' => Some((x as i32, y as i32)),
                    _ => None
                }))
            .collect::<HashSet<_>>();

        let starting_position = s.lines().enumerate()
            .filter_map(|(y, row)| match row.find('^') {
                Some(x) => Some((x as i32, y as i32)),
                None => None
            })
            .next()
            .unwrap();

        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;

        Ok(Grid { obstacles, starting_position, width, height })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn move_in_direction(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1)
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}
