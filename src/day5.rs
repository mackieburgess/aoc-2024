use std::{cmp::Ordering, fs};

fn parse_rules_and_updates(input: String) -> Option<(Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    if let Some((rules, updates)) = input.split_once("\n\n") {
        // X|Y => (X, Y)
        let rules = rules
            .lines()
            .filter_map(|line| {
                if let Some((left, right)) = line.split_once("|") {
                    left.parse().ok().zip(right.parse().ok())
                } else {
                    None
                }
            }).collect::<Vec<_>>();

        // X,Y,Z => [X, Y, Z]
        let updates = updates
            .lines()
            .map(|line| {
                line
                    .split(",")
                    .filter_map(|number| number.parse().ok())
                    .collect::<Vec<usize>>()
            })
            .filter(|line| line.len() > 0)
            .collect::<Vec<_>>();

        Some((rules, updates))
    } else {
        None
    }
}

fn permutations<T>(lst: &Vec<T>) -> Vec<(T, T)>
    where T: Copy
{
    // [1, 2, 3] => [(1, 2), (1, 3), (2, 3)]
    (0..(lst.len() - 1))
        .map(|l| {
            ((l + 1)..lst.len())
                .map(|r| (lst[l], lst[r]))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn correct_update_orders(rules: &Vec<(usize, usize)>, updates: &Vec<Vec<usize>>) -> usize {
    // Take all valid update lists and find the middle element of each.

    updates
        .iter()
        .filter(|update_list| {
            // Get all correct update lists.
            permutations(*update_list)
                .iter()
                .all(|(l, r)| rules.iter().find(|rule| **rule == (*r, *l)) == None)
        })
        .map(|update_list| update_list[update_list.len() / 2])
        .sum()
}

fn corrected_update_orders(rules: Vec<(usize, usize)>, mut updates: Vec<Vec<usize>>) -> usize {
    // Take all invalid update lists, reorder to make them valid, then find the middle element.
    // Since this reordering is based on comparisons, we can just sort by this comparison.

    updates
        .iter_mut()
        .filter(|update_list| {
            // Get all *incorrect* update lists.
            permutations(*update_list)
                .iter()
                .any(|(l, r)| rules.iter().find(|rule| **rule == (*r, *l)) != None)
        })
        .map(|update_list| {
            update_list.sort_by(|a, b| {
                if rules.iter().any(|(l, r)| (r, l) == (a, b)) {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });

            return update_list;
        })
        .map(|update_list| update_list[update_list.len() / 2])
        .sum()
}

fn main() {
    if let Some(input) = fs::read_to_string("data/5.input").ok() {
        if let Some((rules, updates)) = parse_rules_and_updates(input) {
            println!("part one: {}", correct_update_orders(&rules, &updates));
            println!("part two: {}", corrected_update_orders(rules, updates));
        } else {
            panic!("Invalid puzzle input")
        }
    } else {
        panic!("No puzzle input")
    }
}
