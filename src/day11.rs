use std::fs;

fn after_blinks(blinks: usize) -> usize {
    if let Some(input) = fs::read_to_string("data/11.input").ok() {
        input
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok())
            .map(|stone| {
                let mut stone_substack = vec![stone];

                for blink in 0..blinks {
                    println!("{blink}");
                    stone_substack = stone_substack
                        .into_iter()
                        .flat_map(|stone| {
                            return match stone {
                                0 => vec![1],
                                _ if stone.ilog10() % 2 == 1 => {
                                    let power = 10_usize.pow((stone.ilog10() + 1) / 2);
                                    vec![stone / power, stone % power]
                                },
                                _ => vec![stone * 2024]
                            };
                        }).collect();
                }

                stone_substack.len()
            })
            .sum()
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", after_blinks(25));
}
