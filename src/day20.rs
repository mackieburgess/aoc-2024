use std::{collections::{HashMap, HashSet}, fs};

fn build_map(input: String) -> HashMap<(usize, usize), usize> {
    let mut start = None;
    let mut end = None;

    let mut map: HashMap<(usize, usize), usize> = HashMap::new();

    input
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(x, c)| {
                    if c == 'S' { start = Some((x, y)); }
                    if c == 'E' { end = Some((x, y)); }
                    if c != '#' { map.entry((x, y)).or_insert(0); }
                });
        });

    if let Some((start, end)) = start.zip(end) {
        let mut cursor = start;
        let mut counter = 1;

        loop {
            map.entry(cursor).and_modify(|value| *value = counter);
            counter += 1;

            if cursor == end {
                break;
            }

            // The puzzle states there is only ever one path.
            cursor = *[(cursor.0 - 1, cursor.1),
                    (cursor.0 + 1, cursor.1),
                    (cursor.0, cursor.1 - 1),
                    (cursor.0, cursor.1 + 1)
                ].iter()
                .filter(|point| map.contains_key(point))
                .filter(|point| map.get(point) == Some(&0))
                .nth(0)
                .unwrap();
        }
    } else {
        panic!("No start (S) and/or end (E) found")
    }

    map
}

fn manhatten_radius(position: (usize, usize), away: usize) -> Vec<((usize, usize), usize)> {
    (0..=away)
        .flat_map(|y_offset| {
            (0..=(away - y_offset))
                .flat_map(|x_offset| {
                    [
                        (position.0.checked_sub(x_offset).zip(position.1.checked_sub(y_offset)), x_offset + y_offset),
                        (position.0.checked_sub(x_offset).zip(position.1.checked_add(y_offset)), x_offset + y_offset),
                        (position.0.checked_add(x_offset).zip(position.1.checked_sub(y_offset)), x_offset + y_offset),
                        (position.0.checked_add(x_offset).zip(position.1.checked_add(y_offset)), x_offset + y_offset)
                    ]
                })
                .filter_map(|(location, away)| {
                    if let Some(location) = location {
                        Some((location, away))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|(location, _away)| *location != position)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
}

fn shortcuts(
    map: &HashMap<(usize, usize), usize>,
    cheat_distance: usize,
    shortcut_score: usize
) -> usize {
    map
        .iter()
        .flat_map(|(cursor, current_score)| {
            manhatten_radius(*cursor, cheat_distance)
                .into_iter()
                .filter_map(|(location, away)| {
                    if let Some(distant_score) = map.get(&location) {
                        if *distant_score > current_score + away {
                            return Some(distant_score - (current_score + away));
                        }
                    }

                    None
                })
                .collect::<Vec<_>>()
        })
        .filter(|shortcut| *shortcut >= shortcut_score)
        .count()
}

fn main() {
    if let Some(input) = fs::read_to_string("data/20.input").ok() {
        let map = build_map(input);

        println!("part one: {}", shortcuts(&map, 2, 100));
        println!("part two: {}", shortcuts(&map, 20, 100));
    } else {
        panic!("No puzzle input")
    }
}
