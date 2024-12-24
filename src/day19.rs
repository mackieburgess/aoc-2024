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
                if pattern.starts_with(towel) {
                    return Some(number_of_variations(
                        pattern.trim_start_matches(towel).to_string(),
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

fn possible_towel_patterns() -> usize {
    if let Some(input) = fs::read_to_string("data/19.input").ok() {
        if let Some((towels, patterns)) = input.split_once("\n\n") {
            let towels = towels.trim().split(", ").collect::<Vec<_>>();
            let patterns = patterns.trim().split("\n").collect::<Vec<_>>();
            let mut cache = HashMap::new();

            patterns
                .into_iter()
                .filter(|pattern| {
                    number_of_variations(pattern.to_string(), &towels, &mut cache) > 0
                })
                .count()
        } else {
            panic!("Puzzle input isn't split into towels and patterns")
        }
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", possible_towel_patterns());
}
