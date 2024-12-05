use std::fs;

fn find_in_periphery(wordsearch: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let mut directions = vec![];

    if x >= 3 {
        directions.push((-1, 0));
        if y >= 3 { directions.push((-1, -1)); }
        if y <= wordsearch.len() - 4 { directions.push((-1, 1)); }
    }

    if y >= 3 { directions.push((0, -1)); }

    if x <= wordsearch[0].len() - 4 {
        directions.push((1, 0));
        if y >= 3 { directions.push((1, -1)); }
        if y <= wordsearch.len() - 4 { directions.push((1, 1))}
    }

    if y <= wordsearch.len() - 4 { directions.push((0, 1)); }

    directions.iter().filter(|(x_dir, y_dir)| {
        // These casts are safe, due to the if statements.
        let new_x = (x as isize + x_dir) as usize;
        let new_y = (y as isize + y_dir) as usize;

        if wordsearch[new_y][new_x] == 'M' {
            let new_x = (new_x as isize + x_dir) as usize;
            let new_y = (new_y as isize + y_dir) as usize;

            if wordsearch[new_y][new_x] == 'A' {
                let new_x = (new_x as isize + x_dir) as usize;
                let new_y = (new_y as isize + y_dir) as usize;

                if wordsearch[new_y][new_x] == 'S' {
                    return true;
                }
            }
        }

        false
    }).count()
}

fn find_words() -> usize {
    if let Some(input) = fs::read_to_string("data/4.input").ok() {
        let wordsearch: Vec<Vec<char>> = input
            .clone()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, char)| {
                        match char == 'X' {
                            true => find_in_periphery(&wordsearch, x, y),
                            false => 0
                        }
                    }).sum::<usize>()
            }).sum()
    } else {
        panic!("No puzzle input")
    }
}

fn find_crosses() -> usize {
    if let Some(input) = fs::read_to_string("data/4.input").ok() {
        let wordsearch: Vec<Vec<char>> = input
            .clone()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .filter(|(x, char)| {
                        // Ensure we're in reasonable bounds to do y/x +/- 1.
                        if
                            *x > 0 && *x < wordsearch[0].len() - 1 &&
                            y > 0 && y < wordsearch.len() - 1 &&
                            *char == 'A'
                        {
                            // Then brute force the search for M's and S's.
                            if ((wordsearch[y-1][x-1] == 'M' && wordsearch[y+1][x+1] == 'S') ||
                                (wordsearch[y-1][x-1] == 'S' && wordsearch[y+1][x+1] == 'M')) &&
                               ((wordsearch[y+1][x-1] == 'M' && wordsearch[y-1][x+1] == 'S') ||
                                (wordsearch[y+1][x-1] == 'S' && wordsearch[y-1][x+1] == 'M'))
                            {
                                return true;
                            }
                        }

                        false
                    }).count()
            }).sum()
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", find_words());
    println!("part two: {}", find_crosses());
}
