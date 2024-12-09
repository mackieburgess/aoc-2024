use std::fs;

fn parse_equation_line(line: &str) -> Option<(usize, Vec<usize>)> {
    if let Some((total, nums)) = line.split_once(": ") {
        if let Some(total_as_int) = total.parse::<usize>().ok() {
            let nums_as_ints = nums
                .split_whitespace()
                .filter_map(|num| num.parse::<usize>().ok())
                .collect();

            return Some((total_as_int, nums_as_ints));
        }
    }

    None
}

fn can_be_formed<T>(total: usize, components: Vec<usize>, transforms: Vec<T>) -> Option<usize>
    where T: Fn(usize, usize) -> usize + Clone
{
    let mut values = vec![];

    for component in components {
        if values.is_empty() {
            values.push(component);
        } else {
            values = values
                .iter()
                .map(|value| {
                    transforms
                        .iter()
                        .map(|t| t(*value, component))
                })
                .flatten()
                .collect();
        }
    }

    if values.contains(&total) {
        return Some(total);
    } else {
        return None;
    }
}

fn formable(input: String) -> usize {
    let operators = vec![
        |a, b| a + b,
        |a, b| a * b,
    ];

    input
        .lines()
        .filter_map(parse_equation_line)
        .filter_map(|(total, components)| can_be_formed(total, components, operators.clone()))
        .sum()
}

fn formable_with_or(input: String) -> usize {
    let operators = vec![
        |a, b| a + b,
        |a, b| a * b,
        |a, b: usize| a * 10_usize.pow(b.to_string().len() as u32) + b
    ];

    input
        .lines()
        .filter_map(parse_equation_line)
        .filter_map(|(total, components)| can_be_formed(total, components, operators.clone()))
        .sum()
}


fn main() {
    if let Some(input) = fs::read_to_string("data/7.input").ok() {
        println!("part one: {}", formable(input.clone()));
        println!("part two: {}", formable_with_or(input));
    } else {
        panic!("No puzzle input")
    }
}
