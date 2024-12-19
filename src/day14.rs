use std::{cmp::Ordering, fs};

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize)
}

fn robot_quadrants() -> usize {
    if let Some(input) = fs::read_to_string("data/14.input").ok() {
        let size = (101, 103);

        let one_hundred_teleports = input
            .lines()
            .filter_map(|line| {
                match line
                    .replace("p=", "")
                    .replace(" v=", ",")
                    .split(",")
                    .filter_map(|n| n.parse().ok())
                    .collect::<Vec<isize>>()[..]
                {
                    [px, py, vx, vy] => Some(Robot {
                        position: (px, py),
                        velocity: (vx, vy)
                    }),
                    _ => None
                }

            })
            .map(|bot| {
                let new_x = match
                    (bot.position.0 + bot.velocity.0 * 100) % size.0
                {
                    negative if negative < 0 => size.0 + negative,
                    positive => positive
                };

                let new_y = match
                    (bot.position.1 + bot.velocity.1 * 100) % size.1
                {
                    negative if negative < 0 => size.1 + negative,
                    positive => positive
                };

                Robot {
                    position: (new_x, new_y),
                    velocity: bot.velocity
                }
            })
            .collect::<Vec<Robot>>();

        let (a, b, c, d) = one_hundred_teleports
            .iter()
            .map(|bot| {
                match (
                    bot.position.0.cmp(&(size.0 / 2)),
                    bot.position.1.cmp(&(size.1 / 2))
                ) {
                    (Ordering::Less,    Ordering::Less)    => (1, 0, 0, 0),
                    (Ordering::Greater, Ordering::Less)    => (0, 1, 0, 0),
                    (Ordering::Less,    Ordering::Greater) => (0, 0, 1, 0),
                    (Ordering::Greater, Ordering::Greater) => (0, 0, 0, 1),
                    _ => (0, 0, 0, 0)
                }
            })
            .fold((0, 0, 0, 0), |(a1, b1, c1, d1), (a2, b2, c2, d2)|
                (a1 + a2, b1 + b2, c1 + c2, d1 + d2)
            );

        a * b * c * d
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", robot_quadrants());
}

