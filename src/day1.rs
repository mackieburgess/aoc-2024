use std::fs;

fn get_location_lists(lists: String) -> (Vec<usize>, Vec<usize>) {
    lists
        .lines()
        .filter_map(|line| {
            if let Some((left, right)) = line.split_once("   ") {
                // Convert left and right to integers, returning None if they cannot be converted.
                left.parse::<usize>().ok().zip(right.parse::<usize>().ok())
            } else {
                None
            }
        })
        .unzip()
}

fn paired_distances() -> usize {
    // For two lists, sort each of them and zip by difference. Return the total sum of these
    // differences.

    if let Some(input) = fs::read_to_string("data/1.data").ok() {
        let (mut left_list, mut right_list) = get_location_lists(input);

        // Sort both lists.
        left_list.sort();
        right_list.sort();

        return left_list
            .into_iter()
            .zip(right_list)
            .map(|(left, right)| left.abs_diff(right))
            .sum();

    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part 1: {}", paired_distances());
}
