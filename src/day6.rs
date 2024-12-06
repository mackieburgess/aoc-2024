use std::{collections::HashSet, fs};

type PuzzleState = (usize, usize, (usize, usize), HashSet<(usize, usize)>, Direction);

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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

fn steps_to_leave_map(
    width: usize,
    height: usize,
    mut cursor: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    mut direction: Direction
) -> Option<usize> {
    let mut distinct_positions = HashSet::from([cursor]);
    let mut directed_positions = HashSet::from([(cursor, direction)]);

    loop {
        cursor = direction.step(cursor);

        if directed_positions.contains(&(cursor, direction)) {
            // Infinite loop found.
            return None;
        }

        distinct_positions.insert(cursor);
        directed_positions.insert((cursor, direction));

        let going_off_map_edge = match direction {
            Direction::Up => cursor.1 == 0,
            Direction::Right => cursor.0 == width - 1,
            Direction::Down => cursor.1 == height - 1,
            Direction::Left => cursor.0 == 0
        };

        if going_off_map_edge {
            break;
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
                break;
            }

            let place_for_wall = direction.step(cursor);

            if walls.contains(&place_for_wall) {
                direction.turn();
            }
        }
    }

    return Some(distinct_positions.len());
}

fn solve_map((width, height, cursor, walls, direction): PuzzleState) -> usize {
    if let Some(number_of_steps) = steps_to_leave_map(width, height, cursor, &walls, direction) {
        return number_of_steps;
    } else {
        panic!("Infinite loop found!");
    }
}

fn time_loops((width, height, mut cursor, walls, mut direction): PuzzleState) -> usize {
    let mut obstruction_spots = HashSet::new();

    // We need to track positions to make sure we don't place walls where we've been.
    let mut distinct_positions = HashSet::from([cursor]);

    loop {
        if !distinct_positions.contains(&direction.step(cursor)) {
            let mut new_walls = walls.clone();
            new_walls.insert(direction.step(cursor));

            // New direction, to ensure we don't step through the obstruction.
            let mut new_direction = direction.clone();
            new_direction.turn();

            // Account for a wall next to where you're standing.
            if new_walls.contains(&new_direction.step(cursor)) {
                new_direction.turn();
            }

            if steps_to_leave_map(
                width,
                height,
                cursor,
                &new_walls,
                new_direction
            ) == None {
                obstruction_spots.insert(direction.step(cursor));
            }
        }

        cursor = direction.step(cursor);

        distinct_positions.insert(cursor);

        let going_off_map_edge = match direction {
            Direction::Up => cursor.1 == 0,
            Direction::Right => cursor.0 == width - 1,
            Direction::Down => cursor.1 == height - 1,
            Direction::Left => cursor.0 == 0
        };

        if going_off_map_edge {
            break;
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
                break;
            }

            let place_for_wall = direction.step(cursor);

            if walls.contains(&place_for_wall) {
                direction.turn();
            }
        }
    }

    return obstruction_spots.len();
}

fn main() {
    if let Some(input) = fs::read_to_string("data/6.input").ok() {
        let puzzle_state = parse_starting_puzzle_state(input);

        println!("part one: {}", solve_map(puzzle_state.clone()));
        println!("part two: {}", time_loops(puzzle_state));
    } else {
        panic!("No puzzle input")
    }
}
