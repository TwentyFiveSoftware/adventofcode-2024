use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let garden = Garden::from_str(input).unwrap();

    println!("PART 1: {}", part1(&garden));
    println!("PART 2: {}", part2(&garden));
}

fn part1(garden: &Garden) -> usize {
    garden.find_regions().iter()
        .map(|region| region.area() * region.perimeter())
        .sum()
}

fn part2(garden: &Garden) -> usize {
    garden.find_regions().iter()
        .map(|region| region.area() * region.sides())
        .sum()
}

#[derive(Debug)]
struct Garden {
    grid: HashMap<(i32, i32), char>,
    width: i32,
    height: i32,
}

impl Garden {
    fn find_regions(&self) -> Vec<Region> {
        let mut regions = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                if regions.iter().any(|region: &Region| region.plots.contains(&(x, y))) {
                    continue;
                }

                let region = self.find_region((x, y));

                if !regions.contains(&region) {
                    regions.push(region)
                }
            }
        }

        regions
    }

    fn find_region(&self, plot_coords: (i32, i32)) -> Region {
        let mut plots: HashSet<(i32, i32)> = HashSet::new();

        let plant_type = *self.grid.get(&plot_coords).unwrap();

        let mut queue = VecDeque::from(vec![plot_coords]);

        while let Some((x, y)) = queue.pop_front() {
            if plots.contains(&(x, y)) {
                continue;
            }

            plots.insert((x, y));

            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .filter(|coords| self.is_in_garden(*coords))
                .filter(|coords| *self.grid.get(coords).unwrap() == plant_type)
                .filter(|coords| !plots.contains(coords))
                .for_each(|coords| {
                    queue.push_back(coords);
                })
        }

        Region { plots }
    }

    fn is_in_garden(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().enumerate()
            .flat_map(move |(y, row)| row.char_indices()
                .map(move |(x, c)| (x as i32, y as i32, c)))
            .fold(HashMap::new(), |mut grid, (x, y, plant_type)| {
                grid.insert((x, y), plant_type);
                grid
            });

        let width = s.lines().next().unwrap().len() as i32;
        let height = s.lines().count() as i32;

        Ok(Garden { grid, width, height })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Region {
    plots: HashSet<(i32, i32)>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots.clone().into_iter()
            .map(|(x, y)| {
                vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                    .filter(|adjacent_plot| !self.plots.contains(adjacent_plot))
                    .count()
            })
            .sum()
    }

    fn sides(&self) -> usize {
        self.sides_facing_direction(-1, 0) +
            self.sides_facing_direction(1, 0) +
            self.sides_facing_direction(0, -1) +
            self.sides_facing_direction(0, 1)
    }

    fn sides_facing_direction(&self, offset_x: i32, offset_y: i32) -> usize {
        let mut unique_sides: HashSet<Vec<(i32, i32)>> = HashSet::new();

        let sides_facing_direction = self.plots.clone().into_iter()
            .filter(|(x, y)| !self.plots.contains(&(*x + offset_x, *y + offset_y)))
            .collect::<HashSet<(i32, i32)>>();

        sides_facing_direction.iter().for_each(|coords| {
            let mut side = Self::find_connected_plots(*coords, &sides_facing_direction).into_iter().collect::<Vec<(i32, i32)>>();
            side.sort();
            unique_sides.insert(side);
        });

        unique_sides.len()
    }

    fn find_connected_plots(start_coords: (i32, i32), plots: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
        let mut connected_plots: HashSet<(i32, i32)> = HashSet::new();

        let mut queue = VecDeque::from(vec![start_coords]);

        while let Some((x, y)) = queue.pop_front() {
            if connected_plots.contains(&(x, y)) {
                continue;
            }

            connected_plots.insert((x, y));

            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                .filter(|coords| plots.contains(coords))
                .filter(|coords| !connected_plots.contains(coords))
                .for_each(|coords| {
                    queue.push_back(coords);
                })
        }

        connected_plots
    }
}
