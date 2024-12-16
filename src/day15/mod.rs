use std::collections::{HashSet};
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let [raw_warehouse, raw_moves] = input.split("\n\n").collect::<Vec<&str>>()[..] else { return; };
    let moves = raw_moves.chars().map(|c| MoveDirection::from(c)).collect::<Vec<MoveDirection>>();

    println!("PART 1: {}", part1(raw_warehouse, &moves));
    println!("PART 2: {}", part2(raw_warehouse, &moves));
}

fn part1(raw_warehouse: &str, moves: &Vec<MoveDirection>) -> i32 {
    let warehouse = Warehouse::from_str(raw_warehouse).unwrap();
    warehouse.with_moves_applied(moves).gps_sum()
}

fn part2(raw_warehouse: &str, moves: &Vec<MoveDirection>) -> i32 {
    let warehouse = Warehouse::from_str(&raw_warehouse
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.")
    ).unwrap();

    warehouse.with_moves_applied(moves).gps_sum()
}

#[derive(Clone)]
struct Warehouse {
    walls: HashSet<(i32, i32)>,
    boxes: Vec<Box>,
    robot_position: (i32, i32),
}

impl Warehouse {
    fn with_moves_applied(&self, moves: &Vec<MoveDirection>) -> Warehouse {
        let mut warehouse = self.clone();

        for direction in moves {
            let warehouse_safe = warehouse.clone();

            warehouse.move_robot(&direction);

            if !warehouse.is_valid() {
                warehouse = warehouse_safe;
            }
        }

        warehouse
    }

    fn move_robot(&mut self, direction: &MoveDirection) {
        let new_robot_position = direction.apply(self.robot_position);

        self.move_box(new_robot_position, direction);

        self.robot_position = new_robot_position;
    }

    fn move_box(&mut self, position: (i32, i32), direction: &MoveDirection) {
        let Some(warehouse_box) = self.get_box_at(position) else { return; };

        warehouse_box.coords_after_move(direction).into_iter().for_each(|new_position| {
            self.move_box(new_position, direction);
        });

        self.get_box_at(position).unwrap().move_in_direction(direction);
    }

    fn get_box_at(&mut self, position: (i32, i32)) -> Option<&mut Box> {
        self.boxes.iter_mut().find(|warehouse_box| warehouse_box.coords.contains(&position))
    }

    fn is_valid(&self) -> bool {
        !self.walls.contains(&self.robot_position) &&
            self.boxes.iter().all(|warehouse_box| !warehouse_box.coords.contains(&self.robot_position)) &&
            self.boxes.iter().all(|warehouse_box| warehouse_box.coords.iter().all(|position| !self.walls.contains(position)))
    }

    fn gps_sum(&self) -> i32 {
        self.boxes.iter().map(|warehouse_box| warehouse_box.gps()).sum()
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = HashSet::new();
        let mut boxes = Vec::new();
        let mut robot_position = (0, 0);

        s.lines().enumerate().for_each(|(y, row)| row.char_indices().for_each(|(x, c)| {
            match c {
                '#' => {
                    walls.insert((x as i32, y as i32));
                }
                'O' => {
                    boxes.push(Box { coords: vec![(x as i32, y as i32)] });
                }
                '[' => {
                    boxes.push(Box { coords: vec![(x as i32, y as i32), (x as i32 + 1, y as i32)] });
                }
                '@' => {
                    robot_position = (x as i32, y as i32);
                }
                _ => {}
            }
        }));

        Ok(Warehouse { walls, boxes, robot_position })
    }
}

enum MoveDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    fn apply(&self, (x, y): (i32, i32)) -> (i32, i32) {
        match self {
            MoveDirection::None => (x, y),
            MoveDirection::Up => (x, y - 1),
            MoveDirection::Down => (x, y + 1),
            MoveDirection::Left => (x - 1, y),
            MoveDirection::Right => (x + 1, y),
        }
    }
}

impl From<char> for MoveDirection {
    fn from(value: char) -> Self {
        match value {
            '^' => MoveDirection::Up,
            'v' => MoveDirection::Down,
            '<' => MoveDirection::Left,
            '>' => MoveDirection::Right,
            _ => MoveDirection::None
        }
    }
}

#[derive(Clone)]
struct Box {
    coords: Vec<(i32, i32)>,
}

impl Box {
    fn move_in_direction(&mut self, direction: &MoveDirection) {
        self.coords = self.coords.iter().map(|position| direction.apply(*position)).collect();
    }

    fn coords_after_move(&self, direction: &MoveDirection) -> Vec<(i32, i32)> {
        self.coords.iter()
            .map(|position| direction.apply(*position))
            .filter(|position| !self.coords.contains(position))
            .collect()
    }

    fn gps(&self) -> i32 {
        let (x, y) = self.coords[0];
        y * 100 + x
    }
}
