use std::collections::{HashMap};
use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let circuit = Circuit::from_str(input).unwrap();

    println!("PART 1: {}", part1(&circuit));
    println!("PART 2: {}", part2());
}

fn part1(circuit: &Circuit) -> u64 {
    circuit.calculate()
}

fn part2() -> usize {
    0
}

struct Circuit {
    outputs: Vec<Wire>,
}

struct Gate {
    operation: Operation,
    input1: Box<Wire>,
    input2: Box<Wire>,
}

enum Wire {
    Literal(bool),
    Gate(Box<Gate>),
}

#[derive(Copy, Clone)]
enum Operation {
    AND,
    OR,
    XOR,
}

impl Circuit {
    fn calculate(&self) -> u64 {
        self.outputs.iter().enumerate()
            .fold(0, |result, (i, wire)| {
                let bit = if wire.calculate() { 1 } else { 0 };
                result | (bit << i)
            })
    }
}

impl Wire {
    fn calculate(&self) -> bool {
        match self {
            Wire::Literal(value) => *value,
            Wire::Gate(gate) => gate.calculate()
        }
    }
}

impl Gate {
    fn calculate(&self) -> bool {
        self.operation.apply(self.input1.calculate(), self.input2.calculate())
    }
}

impl Operation {
    fn apply(&self, left_operand: bool, right_operand: bool) -> bool {
        match self {
            Operation::AND => left_operand && right_operand,
            Operation::OR => left_operand || right_operand,
            Operation::XOR => (left_operand && !right_operand) || (!left_operand && right_operand)
        }
    }
}

impl FromStr for Circuit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [raw_literals, raw_gates] = s.split("\n\n").collect::<Vec<&str>>()[..] else { return Err(()); };

        let literals = raw_literals.lines().fold(HashMap::new(), |mut map, line| {
            map.insert(&line[..3], line.chars().nth(5).unwrap() == '1');
            map
        });

        let gate_regex = Regex::new(r"^(.{3}) (AND|OR|XOR) (.{3}) -> (.{3})$").unwrap();

        let gates = raw_gates.lines().fold(HashMap::new(), |mut map, line| {
            let (_, [input1, raw_operation, input2, output]) = gate_regex.captures(line).unwrap().extract();
            map.insert(output, (input1, input2, Operation::from_str(raw_operation).unwrap()));
            map
        });

        fn build_wire(output: &str, gates: &HashMap<&str, (&str, &str, Operation)>, literals: &HashMap<&str, bool>) -> Wire {
            if literals.contains_key(output) {
                return Wire::Literal(*literals.get(output).unwrap());
            }

            let (input1, input2, operation) = gates.get(output).unwrap();

            Wire::Gate(Box::new(Gate {
                operation: *operation,
                input1: Box::new(build_wire(input1, gates, literals)),
                input2: Box::new(build_wire(input2, gates, literals)),
            }))
        }

        let mut outputs_with_z = gates.keys()
            .filter(|key| key.starts_with("z"))
            .map(|output| output.to_string())
            .collect::<Vec<String>>();
        outputs_with_z.sort();

        let outputs = outputs_with_z.into_iter()
            .map(|output| build_wire(&output, &gates, &literals))
            .collect::<Vec<Wire>>();

        Ok(Circuit { outputs })
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Operation::AND),
            "OR" => Ok(Operation::OR),
            "XOR" => Ok(Operation::XOR),
            &_ => Err(())
        }
    }
}
