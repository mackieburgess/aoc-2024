use std::fs;

fn get_location_lists(lists: String) -> (Vec<usize>, Vec<usize>) {
    lists
        .lines()
        .filter_map(|line| {
            if let Some((left, right)) = line.split_once("   ") {
                // Convert left and right to integers and zip them together.
                left.parse::<usize>().ok().zip(right.parse::<usize>().ok())
            } else {
                None
            }
        })
        .unzip()
}

fn paired_differences(input: String) -> usize {
    // Compare the difference between each adjacent element of two sorted lists.

    let (mut left_list, mut right_list) = get_location_lists(input);

    // Sort both lists.
    left_list.sort();
    right_list.sort();

    return left_list
        .into_iter()
        .zip(right_list)
        .map(|(left, right)| left.abs_diff(right))
        .sum();
}

fn matched_occurrences(input: String) -> usize {
    // Multiply each number in the left list by how many times it appears in the right list.

    let (left_list, right_list) = get_location_lists(input);

    return left_list
        .iter()
        .map(|left_id| {
            left_id * right_list
                .clone()
                .into_iter()
                .filter(|right_id| left_id == right_id)
                .count()
        }).sum();
}

fn main() {
    if let Some(input) = fs::read_to_string("data/1.data").ok() {
        println!("part 1: {}", paired_differences(input.clone()));
        println!("part 2: {}", matched_occurrences(input));
    } else {
        panic!("No puzzle input")
    }
}
