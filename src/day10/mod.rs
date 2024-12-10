use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let map = Map::from_str(input).unwrap();

    println!("PART 1: {}", part1(&map));
    println!("PART 2: {}", part2(&map));
}

fn part1(map: &Map) -> usize {
    map.trail_heads().into_iter()
        .map(|trail_head| map.calculate_trail_head_score(trail_head))
        .sum()
}

fn part2(map: &Map) -> usize {
    map.trail_heads().into_iter()
        .map(|trail_head| map.calculate_trail_head_rating(trail_head))
        .sum()
}

struct Map {
    grid: HashMap<(i32, i32), u8>,
    width: i32,
    height: i32,
}

impl Map {
    fn trail_heads(&self) -> Vec<(i32, i32)> {
        self.grid.iter()
            .filter_map(|(coords, height)| if *height == 0 { Some(*coords) } else { None })
            .collect::<Vec<(i32, i32)>>()
    }

    fn calculate_trail_head_score(&self, trail_head: (i32, i32)) -> usize {
        self.find_all_trails_for_trail_head(trail_head).into_iter()
            .map(|trail| *trail.last().unwrap())
            .collect::<HashSet<(i32, i32)>>()
            .len()
    }

    fn calculate_trail_head_rating(&self, trail_head: (i32, i32)) -> usize {
        self.find_all_trails_for_trail_head(trail_head).len()
    }

    fn find_all_trails_for_trail_head(&self, trail_head: (i32, i32)) -> Vec<Vec<(i32, i32)>> {
        let mut queue = VecDeque::from(vec![vec![trail_head]]);

        let mut trails: Vec<Vec<(i32, i32)>> = vec![];

        while let Some(path) = queue.pop_front() {
            let (x, y) = *path.last().unwrap();

            let height = *self.grid.get(&(x, y)).unwrap();
            if height == 9 {
                trails.push(path);
                continue;
            }

            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .filter(|coords| self.is_on_map(*coords))
                .filter(|coords| *self.grid.get(coords).unwrap() == height + 1)
                .for_each(|coords| {
                    let mut new_path = path.clone();
                    new_path.push(coords);

                    queue.push_back(new_path);
                })
        }

        trails
    }

    fn is_on_map(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().enumerate()
            .flat_map(move |(y, row)| row.char_indices()
                .map(move |(x, c)| (x as i32, y as i32, c.to_digit(10).unwrap() as u8)))
            .fold(HashMap::new(), |mut grid, (x, y, height)| {
                grid.insert((x, y), height);
                grid
            });

        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;

        Ok(Map { grid, width, height })
    }
}
