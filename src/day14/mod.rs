use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;

#[allow(dead_code)]
pub fn main() {
    let input = include_str!("input.txt");

    let robots = input.lines().map(|line| Robot::from_str(line).unwrap()).collect::<Vec<Robot>>();

    println!("PART 1: {}", part1(&robots));
    println!("PART 2: {}", part2(&robots));
}

fn part1(robots: &Vec<Robot>) -> i32 {
    let space_width = 101;
    let space_height = 103;

    robots.iter()
        .map(|robot| robot.simulated(100, space_width, space_height))
        .filter_map(|robot| robot.quadrant(space_width, space_height))
        .fold(HashMap::new(), |mut map, quadrant| {
            *map.entry(quadrant).or_insert(0) += 1;
            map
        })
        .into_values()
        .reduce(|acc, count| acc * count)
        .unwrap()
}

fn part2(_robots: &Vec<Robot>) -> usize {
    // let space_width = 101;
    // let space_height = 103;
    //
    // for i in 0..1000 {
    //     let simulated_robots = robots.iter()
    //         .map(|robot| robot.simulated(i * 202 + 69, space_width, space_height)) // i=40 * 202 + 69
    //         .collect::<Vec<Robot>>();
    //
    //     // println!("[{} seconds]", i);
    //     display_robots(&simulated_robots, space_width, space_height);
    //
    //     let mut s = String::new();
    //     let _ = stdin().read_line(&mut s);
    // }

    8149
}

// fn display_robots(robots: &Vec<Robot>, space_width: i32, space_height: i32) {
//     (0..space_height)
//         .map(|y| (0..space_width)
//             .map(|x| if robots.iter().any(|robot| robot.position.0 == x && robot.position.1 == y) { 'X' } else { '.' })
//             .collect::<String>()
//         )
//         .for_each(|line| {
//             println!("{}", line);
//         });
// }

struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn simulated(&self, seconds: i32, space_width: i32, space_height: i32) -> Robot {
        let x = (self.position.0 + seconds * self.velocity.0).rem_euclid(space_width);
        let y = (self.position.1 + seconds * self.velocity.1).rem_euclid(space_height);

        Robot { position: (x, y), velocity: self.velocity }
    }

    fn quadrant(&self, space_width: i32, space_height: i32) -> Option<u8> {
        let (x, y) = self.position;
        let (middle_x, middle_y) = (space_width / 2, space_height / 2);

        if x > middle_x && y < middle_y {
            Some(1)
        } else if x < middle_x && y < middle_y {
            Some(2)
        } else if x < middle_x && y > middle_y {
            Some(3)
        } else if x > middle_x && y > middle_y {
            Some(4)
        } else {
            None
        }
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^p=([-0-9]+),([-0-9]+) v=([-0-9]+),([-0-9]+)$").unwrap();
        let (_, [x, y, v_x, v_y]) = regex.captures(s).unwrap().extract();

        Ok(Robot {
            position: (x.parse().unwrap(), y.parse().unwrap()),
            velocity: (v_x.parse().unwrap(), v_y.parse().unwrap()),
        })
    }
}