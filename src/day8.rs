use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[allow(dead_code)]
fn combinations<T: Clone>(obj: Vec<T>) -> Vec<(T, T)> {
    // [1, 2, 3] => [(1 2), (1 3), (2 1), (2 3), (3 1) (3 2)]
    (0..obj.len())
        .map(|i| {
            (0..obj.len())
                .filter(|j| i != *j)
                .map(|j| (obj[i].clone(), obj[j].clone()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn parse_map(input: String) -> HashMap<char, Vec<(isize, isize)>> {
    let mut towers = HashMap::new();

    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, char)| match char {
                    '.' => None,
                    _ => Some((char, (x as isize, y as isize))),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .for_each(|(char, location)| {
            towers
                .entry(char)
                .and_modify(|locations: &mut Vec<_>| locations.push(location))
                .or_insert(vec![location]);
        });

    return towers;
}

fn antinodes_in_range() -> usize {
    if let Some(input) = fs::read_to_string("data/8.input").ok() {
        let width = input.clone().lines().next().unwrap().chars().count() as isize;
        let height = input.clone().lines().count() as isize;

        parse_map(input)
            .into_values()
            .map(|mesh| {
                combinations(mesh)
                    .iter()
                    .map(|(fst, snd)| ((snd.0 + (snd.0 - fst.0)), (snd.1 + (snd.1 - fst.1))))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .filter(|antinode: &(isize, isize)| {
                antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < height
            })
            .collect::<HashSet<_>>()
            .len()
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", antinodes_in_range());
}
