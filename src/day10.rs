use std::{collections::HashSet, fs};

fn destinations(map: &Vec<Vec<usize>>, cursor: (usize, usize)) -> Vec<(usize, usize)> {
    if let Some(current_height) = map
        .get(cursor.1)
        .and_then(|line| line.get(cursor.0))
    {
        if *current_height == 9 {
            return vec![cursor];
        }

        let mut paths = vec![];

        if cursor.0 > 0             { paths.push((cursor.0 - 1, cursor.1)); }
        if cursor.1 > 0             { paths.push((cursor.0, cursor.1 - 1)); }
        if cursor.0 < map.len() - 1 { paths.push((cursor.0 + 1, cursor.1)); }
        if cursor.1 < map.len() - 1 { paths.push((cursor.0, cursor.1 + 1)); }

        paths
            .into_iter()
            .filter(|path| map[path.1][path.0] == current_height + 1)
            .map(|path| destinations(map, path))
            .flatten()
            .collect()
    } else {
        panic!("Off the edge");
    }
}

fn sum_of_trailhead_score() -> usize {
    if let Some(input) = fs::read_to_string("data/10.input").ok() {
        let map: Vec<Vec<usize>> = input
            .lines()
            .map(|line|
                line.chars().filter_map(|c| c.to_string().parse().ok()).collect()
            )
            .collect();

        let trailheads = map
            .iter()
            .enumerate()
            .map(|(y, line)|
                line
                    .iter()
                    .enumerate()
                    .filter(|(_, height)| **height == 0)
                    .map(|(x, _)| (x, y))
                    .collect::<Vec<_>>()
            )
            .flatten()
            .collect::<Vec<_>>();

        trailheads
            .into_iter()
            .map(|trailhead|
                destinations(&map, trailhead)
                    .iter()
                    .collect::<HashSet<_>>()
                    .len()
            ).sum()
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", sum_of_trailhead_score());
}
