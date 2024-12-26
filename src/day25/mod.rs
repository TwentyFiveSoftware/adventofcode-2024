use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let schematics = input.split("\n\n")
        .map(|schematic| Schematic::from_str(schematic).unwrap())
        .collect::<Vec<Schematic>>();

    let locks = schematics.clone().into_iter()
        .filter(|schematic| schematic.is_lock)
        .collect::<Vec<Schematic>>();

    let keys = schematics.into_iter()
        .filter(|schematic| !schematic.is_lock)
        .collect::<Vec<Schematic>>();

    locks.into_iter()
        .map(|lock| keys.iter().filter(|key| lock.does_key_fit_lock(key)).count())
        .sum()
}

#[derive(Clone)]
struct Schematic {
    is_lock: bool,
    pins: [i8; 5],
}

impl Schematic {
    fn does_key_fit_lock(&self, key: &Schematic) -> bool {
        self.pins.iter().zip(key.pins).all(|(lock_pin, key_pin)| key_pin + lock_pin <= 5)
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_lock = s.lines().next().unwrap().starts_with("#");

        let mut lines = s.lines().collect::<Vec<&str>>();
        if !is_lock {
            lines.reverse();
        }

        let pins = lines.into_iter().enumerate()
            .fold([0i8; 5], |mut pins, (i, line)| {
                for (pin_index, c) in line.char_indices() {
                    if c == '#' {
                        pins[pin_index] = i as i8;
                    }
                }

                pins
            });

        Ok(Schematic { is_lock, pins })
    }
}
