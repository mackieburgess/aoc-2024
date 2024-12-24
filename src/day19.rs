use std::{collections::HashMap, fs};

fn number_of_variations(
    pattern: String,
    towels: &Vec<&str>,
    cache: &mut HashMap<String, usize>
) -> usize {
    if let Some(variations) = cache.get(&pattern) {
        return *variations;
    }

    if pattern.chars().count() == 0 {
        return 1;
    } else {
        let variations = towels
            .iter()
            .filter_map(|towel| {
                if let Some(("", rhs)) = pattern.split_once(towel) {
                    return Some(number_of_variations(
                        rhs.to_string(),
                        towels,
                        cache
                    ));
                } else {
                    None
                }
            })
            .sum();

        cache.entry(pattern).or_insert(variations);

        variations
    }
}

fn possible_towel_patterns(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    let mut cache = HashMap::new();

    patterns
        .iter()
        .filter(|pattern| {
            number_of_variations(pattern.to_string(), towels, &mut cache) > 0
        })
        .count()
}

fn possible_towel_ways(towels: &Vec<&str>, patterns: &Vec<&str>) -> usize {
    let mut cache = HashMap::new();

    patterns
        .iter()
        .map(|pattern| {
            number_of_variations(pattern.to_string(), towels, &mut cache)
        })
        .sum()
}

fn main() {
    if let Some(input) = fs::read_to_string("data/19.input").ok() {
        if let Some((towels, patterns)) = input.split_once("\n\n") {
            let towels = towels.trim().split(", ").collect::<Vec<_>>();
            let patterns = patterns.trim().split("\n").collect::<Vec<_>>();

            println!("part one: {}", possible_towel_patterns(&towels, &patterns));
            println!("part one: {}", possible_towel_ways(&towels, &patterns));
        } else {
            panic!("Puzzle input isn't split into towels and patterns")
        }
    } else {
        panic!("No puzzle input")
    }
}
