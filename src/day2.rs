use std::fs;

fn split_into_numbers(input: &str) -> Vec<usize> {
    // Take a line and split into whitespace separated numbers.
    input.split_whitespace().filter_map(|c| c.parse().ok()).collect()
}

fn is_safe(report: &Vec<usize>) -> bool {
    // Check:
    // - Strictly increasing or strictly decreasing.
    // - Each change is a small step.
    // - Line isn't empty (as we rely heavily on `all`)
    (report.windows(2).all(|w| w[0] > w[1]) || report.windows(2).all(|w| w[0] < w[1]))
        && report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
        && report.len() > 0
}

fn safe_reports(input: String) -> usize {
    // Count reports which pass the `is_safe` check.
    input
        .lines()
        .map(split_into_numbers)
        .filter(|line| is_safe(line))
        .count()
}

fn tolerated_reports(input: String) -> usize {
    // Additionally, count reports which only fail because of one value.
    input
        .lines()
        .map(split_into_numbers)
        .filter(|line| {
            if !is_safe(line) {
                // Iterate through each element, try removing it and checking for safety.
                (0..line.len()).any(|idx| {
                    let mut new_line = line.clone();
                    new_line.remove(idx);

                    is_safe(&new_line)
                })
            } else {
                return true;
            }
        }).count()
}

fn main() {
    if let Some(input) = fs::read_to_string("data/2.input").ok() {
        println!("part one: {}", safe_reports(input.clone()));
        println!("part two: {}", tolerated_reports(input));
    }
}
