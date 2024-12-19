use std::{collections::HashSet, fs};

enum Direction {
    Up,
    Right,
    Down,
    Left
}

fn perform_move(
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
    cursor: &mut (usize, usize),
    direction: Direction
) {
    let shift = match direction {
        Direction::Up    => |(x, y): (usize, usize)| (x, y - 1),
        Direction::Right => |(x, y): (usize, usize)| (x + 1, y),
        Direction::Down  => |(x, y): (usize, usize)| (x, y + 1),
        Direction::Left  => |(x, y): (usize, usize)| (x - 1, y)
    };

    let mut pointer = shift(cursor.clone());
    let mut boxes_to_move = vec![];

    loop {
        match (walls.contains(&pointer), boxes.contains(&pointer)) {
            (true, _) => {
                // Found a wall before finding space: damn.
                break;
            }
            (false, true) => {
                // Found a box! Add it to the move list.
                boxes_to_move.push(pointer);
            }
            (false, false) => {
                // Move box.
                boxes_to_move.iter().for_each(|lf| { boxes.remove(lf); });
                boxes_to_move
                    .iter()
                    .map(|lf| shift(*lf))
                    .for_each(|lf| { boxes.insert(lf); });

                *cursor = shift(*cursor);
                break;
            },
        }

        pointer = shift(pointer);
    }
}

fn box_positions() -> usize {
    if let Some(input) = fs::read_to_string("data/15.input").ok() {
        if let Some((map, instructions)) = input.split_once("\n\n") {
            let mut walls = HashSet::new();
            let mut boxes = HashSet::new();
            let mut cursor = None;

            map
                .lines()
                .enumerate()
                .for_each(|(y, line)| {
                    line
                        .chars()
                        .enumerate()
                        .for_each(|(x, c)| {
                            match c {
                                '#' => drop(walls.insert((x, y))),
                                'O' => drop(boxes.insert((x, y))),
                                '@' => cursor = Some((x, y)),
                                _ => {}
                            };
                        });
                });

            if let Some(mut cursor) = cursor {
                instructions
                    .chars()
                    .for_each(|instruction| {
                        let direction = match instruction {
                            '^' => Some(Direction::Up),
                            '>' => Some(Direction::Right),
                            'v' => Some(Direction::Down),
                            '<' => Some(Direction::Left),
                            _   => None
                        };

                        if let Some(direction) = direction {
                            perform_move(&walls, &mut boxes, &mut cursor, direction);
                        }
                    });

                boxes
                    .iter()
                    .map(|(x, y)| x + 100 * y)
                    .sum()
            } else {
                panic!("No cursor (`@` symbol) found in input")
            }
        } else {
            panic!("Input isn't split into map and instructions")
        }
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", box_positions());
}
