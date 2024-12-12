use std::{collections::HashMap, fs};

fn count_blinks(
    cache: &mut HashMap<(usize, usize), usize>,
    stone: usize,
    remaining_blinks: usize
) -> usize {
    if remaining_blinks == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, remaining_blinks)) {
        return *result;
    } else {
        let result = match stone {
            0 => count_blinks(cache, 1, remaining_blinks - 1),
            _ if stone.ilog10() % 2 == 1 => {
                let power = 10_usize.pow((stone.ilog10() + 1) / 2);
                count_blinks(cache, stone / power, remaining_blinks - 1)
                    + count_blinks(cache, stone % power, remaining_blinks - 1)
            },
            _ => count_blinks(cache, stone * 2024, remaining_blinks - 1)
        };

        cache.insert((stone, remaining_blinks), result);
        return result;
    }
}

fn after_blinks(blinks: usize) -> usize {
    if let Some(input) = fs::read_to_string("data/11.input").ok() {
        let mut cache = HashMap::new();

        input
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok())
            .map(|stone| count_blinks(&mut cache, stone, blinks))
            .sum()
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", after_blinks(25));
    println!("part two: {}", after_blinks(75));
}
