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

fn can_be_formed((total, components): (usize, Vec<usize>)) -> Option<usize> {
   let mut values = vec![];

   for component in components {
       if values.is_empty() {
           values.push(component);
       } else {
           let add = values.clone().into_iter().map(|value| value + component);
           let multiply = values.clone().into_iter().map(|value| value * component);

           values = add.into_iter().chain(multiply.into_iter()).collect();
       }
   }

   if values.contains(&total) {
       return Some(total);
   } else {
       return None;
   }
}

fn formable_equations() -> usize {
   if let Some(input) = fs::read_to_string("data/7.input").ok() {
       input
           .lines()
           .filter_map(parse_equation_line)
           .filter_map(can_be_formed)
           .sum()
   } else {
       panic!("No puzzle input")
   }
}

fn main() {
   println!("part one: {}", formable_equations());
}
