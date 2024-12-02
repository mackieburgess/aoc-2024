use std::fs;

fn safe_reports() -> usize {
    if let Some(input) = fs::read_to_string("data/2.input").ok() {
        input.lines().map(|line| {
            line.split_whitespace().filter_map(|c| c.parse::<usize>().ok()).collect::<Vec<_>>()
        }).filter(|line| {
            // Strictly increasing or strictly decreasing.
            line.windows(2).all(|w| w[0] > w[1]) || line.windows(2).all(|w| w[0] < w[1])
        }).filter(|line| {
            // Each change is a small step.
            line.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
        }).filter(|line| {
            // Line isn't empty (as we rely heavily on `all`)
            line.len() > 0
        }).count()
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", safe_reports())
}
