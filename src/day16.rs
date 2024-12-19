use std::{collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Heading {
    North,
    East,
    South,
    West
}

impl Heading {
    fn step(&self, (x, y): &(usize, usize)) -> (usize, usize) {
        match self {
            Heading::North => (*x, y - 1),
            Heading::East  => (x + 1, *y),
            Heading::South => (*x, y + 1),
            Heading::West  => (x - 1, *y),
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Heading::North => Heading::West,
            Heading::East  => Heading::North,
            Heading::South => Heading::East,
            Heading::West  => Heading::South,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Heading::North => Heading::East,
            Heading::East  => Heading::South,
            Heading::South => Heading::West,
            Heading::West  => Heading::North
        }
    }
}

fn cheapest_route(
    map: &Vec<Vec<bool>>,
    heading: Heading,
    cursor: (usize, usize),
    end: (usize, usize)
) -> usize {
    let mut cache = HashSet::new();
    let mut agenda = vec![(cursor, heading, 0)];

    while let Some((cursor, heading, score)) = agenda.pop() {
        if cache.contains(&(cursor, heading)) {
            continue;
        }

        cache.insert((cursor, heading));

        if cursor == end {
            return score;
        }

        let step = heading.step(&cursor);

        if !map[step.1][step.0] {
            agenda.push((step, heading, score + 1));
        }

        let left_rotate = heading.rotate_left();
        let right_rotate = heading.rotate_right();

        if !map[left_rotate.step(&cursor).1][left_rotate.step(&cursor).0] {
            agenda.push((cursor, left_rotate, score + 1000));
        }
        if !map[right_rotate.step(&cursor).1][right_rotate.step(&cursor).0] {
            agenda.push((cursor, right_rotate, score + 1000));
        }

        agenda.sort_by(|a, b| b.2.cmp(&a.2));
    }

    panic!("Agenda exhausted with no solution")
}

fn foo() -> usize {
    if let Some(input) = fs::read_to_string("data/16.input").ok() {
        let mut cursor = None;
        let mut end = None;

        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        match c {
                            '#' => true,
                            'S' => { cursor = Some((x, y)); false },
                            'E' => { end = Some((x, y)); false },
                            _ => false
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();

        if let Some((cursor, end)) = cursor.zip(end) {
            cheapest_route(&map, Heading::East, cursor, end)
        } else {
            panic!("No start and/or end point found")
        }
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", foo());
}
