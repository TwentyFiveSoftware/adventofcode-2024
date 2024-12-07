use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let equations = input.lines()
        .map(|line| Equation::from_str(line).unwrap())
        .collect::<Vec<Equation>>();

    println!("PART 1: {}", part1(&equations));
    println!("PART 2: {}", part2(&equations));
}

fn part1(equations: &[Equation]) -> u64 {
    let operators = vec![Operator::Add, Operator::Multiply];

    equations.iter()
        .filter(|equation| equation.is_solvable(&operators))
        .map(|equation| equation.result)
        .sum()
}

fn part2(equations: &Vec<Equation>) -> u64 {
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatenate];

    equations.iter()
        .filter(|equation| equation.is_solvable(&operators))
        .map(|equation| equation.result)
        .sum()
}

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    fn is_solvable(&self, possible_operators: &[Operator]) -> bool {
        self.calculate_possible_results(possible_operators).contains(&self.result)
    }

    fn calculate_possible_results(&self, possible_operators: &[Operator]) -> Vec<u64> {
        Self::calculate_possible_results_with(vec![self.numbers[0]], &self.numbers[1..], possible_operators)
    }

    fn calculate_possible_results_with(intermediate_results: Vec<u64>, remaining_numbers: &[u64], possible_operators: &[Operator]) -> Vec<u64> {
        if remaining_numbers.is_empty() {
            return intermediate_results;
        }

        let results = intermediate_results.iter()
            .flat_map(|intermediate_result| possible_operators.iter()
                .map(|operator| operator.apply(*intermediate_result, remaining_numbers[0])))
            .collect();

        Self::calculate_possible_results_with(results, &remaining_numbers[1..], possible_operators)
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [result, numbers] = s.split(": ").collect::<Vec<&str>>()[..] else { return Err(()); };

        Ok(Equation {
            result: result.parse().unwrap(),
            numbers: numbers.split(" ").map(|number| number.parse().unwrap()).collect(),
        })
    }
}

enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match &self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concatenate => left * 10u64.pow(right.ilog10() + 1) + right,
        }
    }
}
