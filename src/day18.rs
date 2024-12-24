use std::{collections::HashSet, fs};

fn valid_steps(position: (usize, usize), size: usize) -> Vec<(usize, usize)> {
    let mut rv = vec![];

    if position.0 != 0        { rv.push((position.0 - 1, position.1)); }
    if position.1 != 0        { rv.push((position.0, position.1 - 1)); }
    if position.0 != size - 1 { rv.push((position.0 + 1, position.1)); }
    if position.1 != size - 1 { rv.push((position.0, position.1 + 1)); }

    rv
}

fn distance_from_goal(cursor: (usize, usize), goal: (usize, usize)) -> usize {
    cursor.0.abs_diff(goal.0) + cursor.1.abs_diff(goal.1)
}

fn shortest_path(coordinate_shower: Vec<(usize, usize)>) -> Option<usize> {
    // Shortest path through the maze after 1024 spots are placed.
    let board_size = 71;
    let goal = (board_size - 1, board_size - 1);

    let mut agenda = vec![((0, 0), 0)];

    let mut cache = HashSet::new();

    while let Some((cursor, steps)) = agenda.pop() {
        if cache.contains(&cursor) {
            continue;
        }

        cache.insert(cursor);

        if cursor == goal {
            return Some(steps);
        }

        let next_moves = valid_steps(cursor, board_size)
            .into_iter()
            .filter_map(|pos| {
                if !coordinate_shower.contains(&pos) {
                    Some((pos, steps + 1))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        agenda.extend(next_moves);

        agenda.sort_by(|a, b| {
            if a.1 == b.1 {
                // manhatten_position
                distance_from_goal(a.0, goal).cmp(&distance_from_goal(b.0, goal))
            } else {
                b.1.cmp(&a.1)
            }
        })
    }

    None
}

fn shortest_kilobyte_path() -> usize {
    if let Some(input) = fs::read_to_string("data/18.input").ok() {
        let coordinate_shower = input
            .lines()
            .filter_map(|line| {
                line
                    .split_once(",")
                    .and_then(|(left, right)|
                        left.parse::<usize>().ok()
                            .zip(right.parse::<usize>().ok()))
            })
            .take(1024)
            .collect::<Vec<(usize, usize)>>();

        if let Some(path_length) = shortest_path(coordinate_shower) {
            path_length
        } else {
            panic!("No valid path found")
        }
    } else {
        panic!("No puzzle input")
    }
}

fn closing_bytes() -> String {
    if let Some(input) = fs::read_to_string("data/18.input").ok() {
        let coordinate_shower = input
            .lines()
            .filter_map(|line| {
                line
                    .split_once(",")
                    .and_then(|(left, right)|
                        left.parse::<usize>().ok()
                            .zip(right.parse::<usize>().ok()))
            })
            .collect::<Vec<(usize, usize)>>();

        let mut low = 0;
        let mut high = coordinate_shower.len() - 1;

        while low <= high {
            let middle = (high + low) / 2;

            if shortest_path(coordinate_shower[0..middle].to_vec()).is_some() {
                if shortest_path(coordinate_shower[0..=middle].to_vec()).is_none() {
                    return format!(
                        "{},{}",
                        coordinate_shower[middle].0,
                        coordinate_shower[middle].1
                    );
                } else {
                    low = middle + 1;
                }
            } else {
                high = middle - 1;
            }
        }

        panic!("There is never a way out")
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", shortest_kilobyte_path());
    println!("part two: {}", closing_bytes());
}
