use std::str::FromStr;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let computer = Computer::from_str(input).unwrap();

    println!("PART 1: {}", part1(computer.clone()));
    println!("PART 2: {}", part2(&computer));
}

fn part1(mut computer: Computer) -> String {
    computer.execute_program();
    computer.get_output()
}

fn part2(computer: &Computer) -> u64 {
    fn find_possible_start_register_a(a: u64, remaining_out: &[u8]) -> Vec<u64> {
        if remaining_out.is_empty() {
            return vec![a];
        }

        let out = remaining_out[0] as u64;
        let mut options = Vec::new();

        for x in 0..8 {
            let new_a = (a << 3) | x;
            let b = new_a % 8;
            let c = new_a >> (x ^ 2);

            if out == (b ^ 2 ^ 7 ^ c) % 8 {
                options.append(&mut find_possible_start_register_a(new_a, &remaining_out[1..]));
            }
        }

        options
    }

    find_possible_start_register_a(0, &computer.program.clone().into_iter().rev().collect::<Vec<u8>>())
        .into_iter().min().unwrap_or(0)
}

#[derive(Clone)]
struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    program: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn execute_program(&mut self) {
        while self.instruction_pointer < self.program.len() {
            let opcode = self.program[self.instruction_pointer];
            let operand = self.program[self.instruction_pointer + 1];

            self.instruction_pointer += 2;

            self.execute_instruction(opcode, operand);
        }
    }

    fn execute_instruction(&mut self, opcode: u8, operand: u8) {
        match opcode {
            0 => {
                self.register_a >>= self.get_combo_operand_value(operand);
            }
            1 => {
                self.register_b ^= operand as u64;
            }
            2 => {
                self.register_b = self.get_combo_operand_value(operand) % 8;
            }
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer = operand as usize;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
            }
            5 => {
                self.output.push((self.get_combo_operand_value(operand) % 8) as u8);
            }
            6 => {
                self.register_b = self.register_a >> self.get_combo_operand_value(operand);
            }
            7 => {
                self.register_c = self.register_a >> self.get_combo_operand_value(operand);
            }
            _ => {}
        }
    }

    fn get_combo_operand_value(&self, operand: u8) -> u64 {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => operand as u64
        }
    }

    fn get_output(&self) -> String {
        self.output.iter().map(|number| number.to_string()).collect::<Vec<String>>().join(",")
    }
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [line1, line2, line3, _, line4] = s.split("\n").collect::<Vec<&str>>()[..] else { return Err(()); };

        Ok(Computer {
            register_a: (&line1[12..]).parse().unwrap(),
            register_b: (&line2[12..]).parse().unwrap(),
            register_c: (&line3[12..]).parse().unwrap(),
            program: (&line4[9..]).split(",").map(|number| number.parse::<u8>().unwrap()).collect(),
            instruction_pointer: 0,
            output: Vec::new(),
        })
    }
}
