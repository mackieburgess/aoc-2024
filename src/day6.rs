use std::{collections::HashSet, fs};

type PuzzleState = (usize, usize, (usize, usize), HashSet<(usize, usize)>, Direction);

enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn(&mut self) {
        match self {
            Direction::Up    => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down  => *self = Direction::Left,
            Direction::Left  => *self = Direction::Up,
        }
    }

    fn step(&self, cursor: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up    => (cursor.0, cursor.1 - 1),
            Direction::Right => (cursor.0 + 1, cursor.1),
            Direction::Down  => (cursor.0, cursor.1 + 1),
            Direction::Left  => (cursor.0 - 1, cursor.1)
        }
    }
}

fn parse_starting_puzzle_state(input: String) -> PuzzleState {
    let height = input.lines().count();
    let width = input.lines().nth(0).unwrap().chars().count();

    let walls = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, char)| { if char == '#' { Some((x, y)) } else { None }})
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>();

    let cursor = input.lines().enumerate().filter_map(|(y, line)| {
        if let Some((x, _char)) = line.chars().enumerate().find(|(_x, char)| *char == '^') {
            return Some((x, y));
        } else {
            None
        }
    }).nth(0).unwrap();

    let direction = Direction::Up;

    return (
        width,
        height,
        cursor,
        walls,
        direction
    );
}

fn steps_to_leave_map() -> usize {
    if let Some(input) = fs::read_to_string("data/6.input").ok() {
        let (
            width,
            height,
            mut cursor,
            walls,
            mut direction
        ) = parse_starting_puzzle_state(input);

        let mut has_left_map = false;
        let mut distinct_positions = HashSet::from([cursor]);

        while !has_left_map {
            cursor = direction.step(cursor);

            distinct_positions.insert(cursor);

            let going_off_map_edge = match direction {
                Direction::Up => cursor.1 == 0,
                Direction::Right => cursor.0 == width - 1,
                Direction::Down => cursor.1 == height - 1,
                Direction::Left => cursor.0 == 0
            };

            if going_off_map_edge {
                has_left_map = true;
                continue;
            }

            let place_for_wall = direction.step(cursor);

            if walls.contains(&place_for_wall) {
                direction.turn();

                // Check that our turn hasn't put us on the verge of going off the map edge, or
                // into another wall.

                let going_off_map_edge = match direction {
                    Direction::Up => cursor.1 == 0,
                    Direction::Right => cursor.0 == width - 1,
                    Direction::Down => cursor.1 == height - 1,
                    Direction::Left => cursor.0 == 0
                };

                if going_off_map_edge {
                    has_left_map = true;
                    continue;
                }

                let place_for_wall = direction.step(cursor);

                if walls.contains(&place_for_wall) {
                    direction.turn();
                }
            }


        }

        return distinct_positions.len();
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", steps_to_leave_map());
}
