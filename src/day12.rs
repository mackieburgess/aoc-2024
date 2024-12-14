use std::{collections::{HashMap, HashSet}, fs};

struct FencedArea {
    area: usize,
    perimeter: usize
}

fn fill_region(
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

                    fill_region(*location, visited, map)
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
        let mut visited = HashSet::new();

        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        (0..map.len())
            .map(|y| {
                (0..map.len())
                    .filter_map(|x| fill_region((x, y), &mut visited, &map))
                    .map(|region| region.perimeter * region.area)
                    .sum::<usize>()
            }).sum()
    } else {
        panic!("No puzzle input")
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum FreeSide {
    Top,
    Bottom,
    Left,
    Right
}

type Region = HashMap<(usize, usize), HashSet<FreeSide>>;

fn parse_region(
    cursor: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    map: &Vec<Vec<char>>
) -> Option<Region> {
    if !visited.contains(&cursor) {
        visited.insert(cursor);

        // Lazily assuming the map is square.
        let size = map.len();
        let plot_type = map[cursor.1][cursor.0];

        let mut free_sides = HashSet::new();
        let mut plots = HashMap::new();

        // Top
        if cursor.1 > 0 && map[cursor.1 - 1][cursor.0] == plot_type {
            if let Some(subregion) = parse_region((cursor.0, cursor.1 - 1), visited, map) {
                plots.extend(subregion);
            }
        } else {
            free_sides.insert(FreeSide::Top);
        }

        // Bottom
        if cursor.1 < size - 1 && map[cursor.1 + 1][cursor.0] == plot_type {
            if let Some(subregion) = parse_region((cursor.0, cursor.1 + 1), visited, map) {
                plots.extend(subregion);
            }
        } else {
            free_sides.insert(FreeSide::Bottom);
        }

        // Left
        if cursor.0 > 0 && map[cursor.1][cursor.0 - 1] == plot_type {
            if let Some(subregion) = parse_region((cursor.0 - 1, cursor.1), visited, map) {
                plots.extend(subregion);
            }
        } else {
            free_sides.insert(FreeSide::Left);
        }

        // Right
        if cursor.0 < size - 1 && map[cursor.1][cursor.0 + 1] == plot_type {
            if let Some(subregion) = parse_region((cursor.0 + 1, cursor.1), visited, map) {
                plots.extend(subregion);
            }
        } else {
            free_sides.insert(FreeSide::Right);
        }


        plots.entry(cursor).or_insert(free_sides);

        return Some(plots);
    }

    None
}

fn count_sides(region: &Region) -> usize {
    //
    //Extremely imperative, because I give up.
    //

    let mut visited_sides: HashSet<((usize, usize), FreeSide)> = HashSet::new();
    let mut sides = 0;

    for (plot, free_sides) in region {
        for side in free_sides {
            if !visited_sides.contains(&(*plot, *side)) {
                visited_sides.insert((*plot, *side));

                sides += 1;

                if *side == FreeSide::Top || *side == FreeSide::Bottom {
                    // Go left/right looking for siblings
                    let mut x = plot.0;
                    while let Some(descending) = x.checked_sub(1) {
                        x = descending;

                        if let Some(sibling) = region.get(&(x, plot.1)) {
                            if !sibling.contains(side) {
                                break;
                            }

                            visited_sides.insert(((x, plot.1), *side));
                        } else {
                            break;
                        }
                    }

                    let mut x = plot.0;
                    while let Some(ascending) = x.checked_add(1) {
                        x = ascending;

                        if let Some(sibling) = region.get(&(x, plot.1)) {
                            if !sibling.contains(side) {
                                break;
                            }

                            visited_sides.insert(((x, plot.1), *side));
                        } else {
                            break;
                        }
                    }
                } else {
                    // Go up/down looking for siblings
                    let mut y = plot.1;
                    while let Some(descending) = y.checked_sub(1) {
                        y = descending;
                        if let Some(sibling) = region.get(&(plot.0, y)) {
                            if !sibling.contains(side) {
                                break;
                            } else {
                                visited_sides.insert(((plot.0, y), *side));
                            }

                        } else {
                            break;
                        }
                    }

                    let mut y = plot.1;
                    while let Some(ascending) = y.checked_add(1) {
                        y = ascending;

                        if let Some(sibling) = region.get(&(plot.0, y)) {
                            if !sibling.contains(side) {
                                break;
                            }

                            visited_sides.insert(((plot.0, y), *side));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    sides
}


fn discount_fencing() -> usize {
    if let Some(input) = fs::read_to_string("data/12.input").ok() {
        let mut visited = HashSet::new();

        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let regions: Vec<Region> = (0..map.len())
            .flat_map(|y| {
                (0..map.len())
                    .filter_map(|x| parse_region((x, y), &mut visited, &map))
                    .collect::<Vec<_>>()
            }).collect();


        regions
            .iter()
            .map(|region| count_sides(region) * region.keys().count())
            .sum()
    } else {
        panic!("No puzzle input")
    }
}





fn main() {
    println!("part one: {}", total_fencing());
    println!("part two: {}", discount_fencing());
}
