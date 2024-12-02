#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap_or_default())
            .map_windows(|[a, b]| b - a))
        .map(is_report_safe)
        .filter(|is_safe| *is_safe)
        .count()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let levels = line.split_whitespace()
                .map(|level| level.parse::<i32>().unwrap_or_default());

            (0..levels.clone().count())
                .map(|i| levels.clone().enumerate()
                    .filter_map(|(index, level)| if index == i { None } else { Some(level) })
                    .map_windows(|[a, b]| b - a)
                    .collect::<Vec<i32>>())
                .any(|report| is_report_safe(report.into_iter()))
        })
        .filter(|is_safe| *is_safe)
        .count()
}

fn is_report_safe(report: impl Iterator<Item=i32> + Clone) -> bool {
    let is_increasing = report.clone().all(|difference| difference >= 1 && difference <= 3);
    let is_decreasing = report.clone().all(|difference| difference <= -1 && difference >= -3);

    is_increasing || is_decreasing
}
