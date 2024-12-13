use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let claw_machines = input.split("\n\n")
        .map(|claw_machine| ClawMachine::from_str(claw_machine).unwrap())
        .collect::<Vec<ClawMachine>>();

    println!("PART 1: {}", part1(&claw_machines));
    println!("PART 2: {}", part2(&claw_machines));
}

fn part1(claw_machines: &Vec<ClawMachine>) -> i64 {
    claw_machines.iter()
        .map(|claw_machine| claw_machine.cost())
        .sum()
}

fn part2(claw_machines: &Vec<ClawMachine>) -> i64 {
    claw_machines.iter()
        .map(|claw_machine| claw_machine.with_price_increase(10000000000000).cost())
        .sum()
}

struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    price: (i64, i64),
}

impl ClawMachine {
    fn cost(&self) -> i64 {
        let button_b_amount = (self.price.1 * self.button_a.0 - self.button_a.1 * self.price.0) /
            (self.button_b.1 * self.button_a.0 - self.button_b.0 * self.button_a.1);

        let button_a_amount = (self.price.0 - button_b_amount * self.button_b.0) / self.button_a.0;

        if self.is_valid_result(button_a_amount, button_b_amount) {
            button_a_amount * 3 + button_b_amount
        } else {
            0
        }
    }

    fn is_valid_result(&self, button_a_amount: i64, button_b_amount: i64) -> bool {
        (self.button_a.0 * button_a_amount + self.button_b.0 * button_b_amount == self.price.0) &&
            (self.button_a.1 * button_a_amount + self.button_b.1 * button_b_amount == self.price.1)
    }

    fn with_price_increase(&self, increase: i64) -> ClawMachine {
        ClawMachine {
            button_a: self.button_a,
            button_b: self.button_b,
            price: (self.price.0 + increase, self.price.1 + increase),
        }
    }
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [line1, line2, line3] = s.split("\n").collect::<Vec<&str>>()[..] else { return Err(()); };

        let button_regex = Regex::new(r"^Button [AB]: X([-+0-9]+), Y([-+0-9]+)$").unwrap();
        let price_regex = Regex::new(r"^Prize: X=([-+0-9]+), Y=([-+0-9]+)$").unwrap();

        let (_, [button_a_x, button_a_y]) = button_regex.captures(line1).unwrap().extract();
        let (_, [button_b_x, button_b_y]) = button_regex.captures(line2).unwrap().extract();
        let (_, [price_x, price_y]) = price_regex.captures(line3).unwrap().extract();

        Ok(ClawMachine {
            button_a: (button_a_x.parse().unwrap(), button_a_y.parse().unwrap()),
            button_b: (button_b_x.parse().unwrap(), button_b_y.parse().unwrap()),
            price: (price_x.parse().unwrap(), price_y.parse().unwrap()),
        })
    }
}
