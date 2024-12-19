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

fn box_positions(map: &str, instructions: &str) -> usize {
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
}

fn perform_wide_move(
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

    let check_box = match direction {
        Direction::Up    => |(x, y): (usize, usize)| (x, y - 1),
        // Right is +2, as it must reach over the "void" half of the boxes.
        Direction::Right => |(x, y): (usize, usize)| (x + 2, y),
        Direction::Down  => |(x, y): (usize, usize)| (x, y + 1),
        Direction::Left  => |(x, y): (usize, usize)| (x - 1, y)
    };

    let mut upper_layer = vec![shift(*cursor)];
    let mut boxes_to_move = HashSet::new();

    loop {
        if upper_layer.iter().any(|unit| walls.contains(unit)) {
            break;
        } else {
            let boxes_in_layer = upper_layer
                .iter()
                .filter_map(|unit| {
                    if boxes.contains(&unit) {
                        Some(*unit)
                    } else if boxes.contains(&(unit.0 - 1, unit.1)) {
                        // Account for the right half of the box "jutting out".
                        Some((unit.0 - 1, unit.1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if boxes_in_layer.len() == 0 {
                boxes_to_move.iter().for_each(|unit| { boxes.remove(unit); });
                boxes_to_move
                    .iter()
                    .map(|unit| shift(*unit))
                    .for_each(|unit| { boxes.insert(unit); });
                *cursor = shift(*cursor);
                break;
            } else {
                boxes_in_layer
                    .iter()
                    .for_each(|unit| { boxes_to_move.insert(*unit); });
                upper_layer = boxes_in_layer
                    .iter()
                    .flat_map(|unit| {
                        let unit_to_check = check_box(*unit);
                        match direction {
                            Direction::Up | Direction::Down =>
                                vec![unit_to_check, (unit_to_check.0 + 1, unit_to_check.1)],
                            _ => vec![unit_to_check],
                        }
                    })
                    .collect();
            }
        }
    }
}

fn wide_box_positions(map: &str, instructions: &str) -> usize {
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
                        '#' => {
                            walls.insert((x * 2, y));
                            walls.insert((x * 2 + 1, y));
                        },
                        'O' => {
                            boxes.insert((x * 2, y));
                        },
                        '@' => cursor = Some((x * 2, y)),
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
                    perform_wide_move(&walls, &mut boxes, &mut cursor, direction);
                }
            });

        boxes
            .iter()
            .map(|(x, y)| x + 100 * y)
            .sum()
    } else {
        panic!("No cursor (`@` symbol) found in input")
    }
}

fn main() {
    if let Some(input) = fs::read_to_string("data/15.input").ok() {
        if let Some((map, instructions)) = input.split_once("\n\n") {
            println!("part one: {}", box_positions(map, instructions));
            println!("part two: {}", wide_box_positions(map, instructions));
        } else {
            panic!("Input isn't split into map and instructions")
        }
    } else {
        panic!("No puzzle input")
    }
}
