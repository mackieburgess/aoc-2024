use std::fs;
use regex::Regex;

fn find_muls(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input).filter_map(|capture| {
        let (_, [mul1, mul2]) = capture.extract();

        match (mul1.parse::<usize>().ok(), mul2.parse::<usize>().ok()) {
            (Some(m1), Some(m2)) => Some(m1 * m2),
            _ => None
        }
    }).sum()
}

fn annoying_muls(input: String) -> usize {
    let re = Regex::new(r"don\'t\(\).*?do\(\)").unwrap();

    // Remove all newlines, add `do()` at the end to fulfill any final capture.
    let input = input.replace("\n", "") + "do()";

    re.split(&input).map(|segment| {
        find_muls(segment)
    }).sum()
}

fn main() {
    if let Some(input) = fs::read_to_string("data/3.input").ok() {
        println!("part one: {}", find_muls(&input));
        println!("part two: {}", annoying_muls(input));
    } else {
        panic!("No puzzle input")
    }
}
