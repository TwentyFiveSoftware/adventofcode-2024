use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    regex.captures_iter(input)
        .map(|captures| {
            let (_, [arg1, arg2]) = captures.extract();

            let a = arg1.parse::<u32>().unwrap_or_default();
            let b = arg2.parse::<u32>().unwrap_or_default();

            a * b
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    const DO_PATTERN: &str = "do()";
    const DONT_PATTERN: &str = "don't()";
    let mul_regex = Regex::new(r"^mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut is_mul_enabled = true;
    let mut sum: u32 = 0;
    let mut i: usize = 0;

    while i < input.len() {
        let substring = &input[i..];

        if substring.starts_with(DO_PATTERN) {
            i += DO_PATTERN.len();
            is_mul_enabled = true;
            continue;
        }

        if substring.starts_with(DONT_PATTERN) {
            i += DONT_PATTERN.len();
            is_mul_enabled = false;
            continue;
        }

        let Some(captures) = mul_regex.captures(substring) else {
            i += 1;
            continue;
        };

        let (capture, [arg1, arg2]) = captures.extract();
        i += capture.len();

        if !is_mul_enabled {
            continue;
        }

        let a = arg1.parse::<u32>().unwrap_or_default();
        let b = arg2.parse::<u32>().unwrap_or_default();
        sum += a * b;
    }

    sum
}
