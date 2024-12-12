use std::{collections::HashSet, fs};

struct FencedArea {
    area: usize,
    perimeter: usize
}

fn flood_fill(
    cursor: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>
) -> Option<FencedArea> {
    if !visited.contains(&cursor) {
        visited.insert(cursor);
        // Lazily assuming the map is square.
        let size = map.len();
        let plot_type = map[cursor.1][cursor.0];

        let mut periphery = vec![];
        if cursor.0 > 0        { periphery.push((cursor.0 - 1, cursor.1)); }
        if cursor.0 < size - 1 { periphery.push((cursor.0 + 1, cursor.1)); }
        if cursor.1 > 0        { periphery.push((cursor.0, cursor.1 - 1)); }
        if cursor.1 < size - 1 { periphery.push((cursor.0, cursor.1 + 1)); }

        let mut perimeter = 4;

        let subregions = periphery
            .iter()
            .filter_map(|location| {
                if map[location.1][location.0] == plot_type {
                    perimeter -= 1;

                    flood_fill(*location, visited, map)
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        Some(FencedArea {
            perimeter: perimeter + subregions.iter().map(|subregion| subregion.perimeter).sum::<usize>(),
            area: 1 + subregions.iter().map(|subregion| subregion.area).sum::<usize>()
        })
    } else {
        None
    }
}

fn total_fencing() -> usize {
    if let Some(input) = fs::read_to_string("data/12.input").ok() {
        // let mut regions = HashMap::new();
        let mut visited = HashSet::new();

        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        (0..map.len())
            .map(|y| {
                (0..map.len())
                    .filter_map(|x| flood_fill((x, y), &mut visited, &map))
                    .map(|region| region.perimeter * region.area)
                    .sum::<usize>()
            }).sum()
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", total_fencing())
}
