use std::fs;
use regex::Regex;

fn find_muls() -> usize {
    if let Some(input) = fs::read_to_string("data/3.input").ok() {
        let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

        re.captures_iter(&input).filter_map(|capture| {
            let (_, [mul1, mul2]) = capture.extract();

            match (mul1.parse::<usize>().ok(), mul2.parse::<usize>().ok()) {
                (Some(m1), Some(m2)) => Some(m1 * m2),
                _ => None
            }
        }).sum()
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", find_muls());
}
