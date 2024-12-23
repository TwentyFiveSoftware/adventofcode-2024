use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let codes = input.lines().map(|line| line.to_string()).collect::<Vec<String>>();

    println!("PART 1: {}", part1(&codes));
    println!("PART 2: {}", part2(&codes));
}

fn part1(codes: &Vec<String>) -> usize {
    calculate_code_complexities(codes, build_keypad_chain(2))
}

fn part2(codes: &Vec<String>) -> usize {
    calculate_code_complexities(codes, build_keypad_chain(25))
}

fn calculate_code_complexities(codes: &Vec<String>, keypad: NumericKeypad) -> usize {
    let mut cache: HashMap<(usize, char, char), usize> = HashMap::new();

    codes.iter()
        .map(|code| {
            let shortest_sequence_length = keypad.clone().find_shortest_sequence_length_to_input_sequence(code, &mut cache);
            let numeric_part_of_code = (&code[..code.len() - 1]).parse::<usize>().unwrap();

            shortest_sequence_length * numeric_part_of_code
        })
        .sum()
}

fn build_keypad_chain(number_of_robot_directional_keypads: usize) -> NumericKeypad {
    let mut current_keypad = DirectionalKeypad { id: 0, controlling_keyboard: None };

    for i in 0..number_of_robot_directional_keypads {
        current_keypad = DirectionalKeypad {
            id: i + 1,
            controlling_keyboard: Some(Box::new(current_keypad)),
        };
    }

    NumericKeypad {
        current_button: 'A',
        controlling_keyboard: current_keypad,
    }
}

#[derive(Clone)]
struct NumericKeypad {
    current_button: char,
    controlling_keyboard: DirectionalKeypad,
}

impl NumericKeypad {
    fn find_shortest_sequence_length_to_input_sequence(&mut self, input_sequence: &str, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
        let mut shortest_sequence_length = 0;

        for next_button in input_sequence.chars() {
            shortest_sequence_length += Self::get_all_shortest_sequences(self.current_button, next_button).into_iter()
                .map(|sequence| self.controlling_keyboard.find_shortest_sequence_length_to_input_sequence(&sequence, cache))
                .min()
                .unwrap();

            self.current_button = next_button;
        }

        shortest_sequence_length
    }

    fn get_all_shortest_sequences(from_button: char, to_button: char) -> HashSet<String> {
        let (from_x, from_y) = Self::button_position(from_button);
        let (to_x, to_y) = Self::button_position(to_button);

        let dx = to_x - from_x;
        let dy = to_y - from_y;

        let mut sequences = HashSet::new();

        if !((from_button == '0' || from_button == 'A') && (to_button == '1' || to_button == '4' || to_button == '7')) {
            sequences.insert(offset_to_sequence(dx, dy, true));
        }

        if !((from_button == '1' || from_button == '4' || from_button == '7') && (to_button == '0' || to_button == 'A')) {
            sequences.insert(offset_to_sequence(dx, dy, false));
        }

        sequences
    }

    fn button_position(button: char) -> (i32, i32) {
        match button {
            '0' => (-1, 0),
            '1' => (-2, -1),
            '2' => (-1, -1),
            '3' => (0, -1),
            '4' => (-2, -2),
            '5' => (-1, -2),
            '6' => (0, -2),
            '7' => (-2, -3),
            '8' => (-1, -3),
            '9' => (0, -3),
            'A' | _ => (0, 0)
        }
    }
}

#[derive(Clone)]
struct DirectionalKeypad {
    id: usize,
    controlling_keyboard: Option<Box<DirectionalKeypad>>,
}

impl DirectionalKeypad {
    fn find_shortest_sequence_length_to_input_sequence(&self, input_sequence: &str, cache: &mut HashMap<(usize, char, char), usize>) -> usize {
        let Some(controlling_keypad) = &self.controlling_keyboard else {
            return input_sequence.len();
        };

        let mut shortest_sequence_length = 0;
        let mut current_button = 'A';

        for button_to_press in input_sequence.chars() {
            let cache_key = (self.id, current_button, button_to_press);

            let length = if cache.contains_key(&cache_key) {
                *cache.get(&cache_key).unwrap()
            } else {
                let length = DirectionalKeypad::get_all_shortest_sequences(current_button, button_to_press).into_iter()
                    .map(|sequence| controlling_keypad.find_shortest_sequence_length_to_input_sequence(&sequence, cache))
                    .min()
                    .unwrap();

                cache.insert(cache_key, length);
                length
            };

            shortest_sequence_length += length;
            current_button = button_to_press;
        }

        shortest_sequence_length
    }

    fn get_all_shortest_sequences(from_button: char, to_button: char) -> HashSet<String> {
        let (from_x, from_y) = Self::button_position(from_button);
        let (to_x, to_y) = Self::button_position(to_button);

        let dx = to_x - from_x;
        let dy = to_y - from_y;

        let mut sequences = HashSet::new();

        if !((from_button == '^' || from_button == 'A') && to_button == '<') {
            sequences.insert(offset_to_sequence(dx, dy, true));
        }

        if !(from_button == '<' && (to_button == '^' || to_button == 'A')) {
            sequences.insert(offset_to_sequence(dx, dy, false));
        }

        sequences
    }

    fn button_position(button: char) -> (i32, i32) {
        match button {
            '^' => (-1, 0),
            '<' => (-2, 1),
            'v' => (-1, 1),
            '>' => (0, 1),
            'A' | _ => (0, 0)
        }
    }
}

fn offset_to_sequence(dx: i32, dy: i32, x_first: bool) -> String {
    let x_char = if dx > 0 { ">" } else { "<" };
    let y_char = if dy > 0 { "v" } else { "^" };

    let x_sequence = x_char.repeat(dx.abs() as usize);
    let y_sequence = y_char.repeat(dy.abs() as usize);

    match x_first {
        true => format!("{}{}A", x_sequence, y_sequence),
        false => format!("{}{}A", y_sequence, x_sequence),
    }
}
