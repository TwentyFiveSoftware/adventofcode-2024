use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from_str(input).unwrap();

    println!("PART 1: {}", part1(&grid));
    println!("PART 2: {}", part2(&grid));
}

fn part1(grid: &Grid) -> usize {
    (0..grid.height)
        .flat_map(move |row_index| (0..grid.width).map(move |column_index| (row_index, column_index)))
        .map(|(row_index, column_index)| grid.calculate_xmas_count_at(row_index, column_index))
        .sum()
}

fn part2(grid: &Grid) -> usize {
    (0..grid.height)
        .flat_map(move |row_index| (0..grid.width).map(move |column_index| (row_index, column_index)))
        .filter(|(row_index, column_index)| grid.is_x_mas_at(*row_index, *column_index))
        .count()
}

struct Grid {
    width: usize,
    height: usize,
    cells: HashMap<(usize, usize), char>,
}

impl Grid {
    fn calculate_xmas_count_at(&self, row_index: usize, column_index: usize) -> usize {
        [
            self.is_horizontal_xmas_at(row_index, column_index),
            self.is_vertical_xmas_at(row_index, column_index),
            self.is_right_diagonal_xmas_at(row_index, column_index),
            self.is_left_diagonal_xmas_at(row_index, column_index)
        ]
            .iter()
            .filter(|is_xmas| **is_xmas)
            .count()
    }

    fn is_horizontal_xmas_at(&self, row_index: usize, column_index: usize) -> bool {
        self.is_xmas([
            (row_index, column_index),
            (row_index, column_index + 1),
            (row_index, column_index + 2),
            (row_index, column_index + 3)
        ])
    }

    fn is_vertical_xmas_at(&self, row_index: usize, column_index: usize) -> bool {
        self.is_xmas([
            (row_index, column_index),
            (row_index + 1, column_index),
            (row_index + 2, column_index),
            (row_index + 3, column_index)
        ])
    }

    fn is_right_diagonal_xmas_at(&self, row_index: usize, column_index: usize) -> bool {
        self.is_xmas([
            (row_index, column_index),
            (row_index + 1, column_index + 1),
            (row_index + 2, column_index + 2),
            (row_index + 3, column_index + 3)
        ])
    }

    fn is_left_diagonal_xmas_at(&self, row_index: usize, column_index: usize) -> bool {
        self.is_xmas([
            (row_index, column_index),
            (row_index - 1, column_index + 1),
            (row_index - 2, column_index + 2),
            (row_index - 3, column_index + 3)
        ])
    }

    fn is_xmas(&self, indices: [(usize, usize); 4]) -> bool {
        "XMAS".char_indices().all(|(i, char)| self.get_char_at(indices[i].0, indices[i].1) == &char) ||
            "SAMX".char_indices().all(|(i, char)| self.get_char_at(indices[i].0, indices[i].1) == &char)
    }

    fn get_char_at(&self, row_index: usize, column_index: usize) -> &char {
        self.cells.get(&(row_index, column_index)).unwrap_or(&'.')
    }

    fn is_x_mas_at(&self, row_index: usize, column_index: usize) -> bool {
        (
            self.get_char_at(row_index, column_index) == &'A'
        ) && (
            (
                self.get_char_at(row_index - 1, column_index - 1) == &'M' &&
                    self.get_char_at(row_index + 1, column_index + 1) == &'S'
            ) || (
                self.get_char_at(row_index - 1, column_index - 1) == &'S' &&
                    self.get_char_at(row_index + 1, column_index + 1) == &'M'
            )
        ) && (
            (
                self.get_char_at(row_index - 1, column_index + 1) == &'M' &&
                    self.get_char_at(row_index + 1, column_index - 1) == &'S'
            ) || (
                self.get_char_at(row_index - 1, column_index + 1) == &'S' &&
                    self.get_char_at(row_index + 1, column_index - 1) == &'M'
            )
        )
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .enumerate()
            .flat_map(move |(row_index, row)| row.char_indices()
                .map(move |(column_index, char)| (row_index, column_index, char)))
            .fold(HashMap::new(), |mut map, (row_index, column_index, char)| {
                map.insert((row_index, column_index), char);
                map
            });

        let width = s.lines().next().unwrap_or_default().len();
        let height = s.lines().count();

        Ok(Grid { width, height, cells })
    }
}
