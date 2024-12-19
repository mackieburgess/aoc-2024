use std::{collections::{HashMap, HashSet}, fs};

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
) -> (usize, usize) {
    let mut cache = HashMap::new();
    let mut agenda = vec![(cursor, heading, 0, HashMap::new())];

    let mut valid_paths: HashMap<((usize, usize), Heading), usize> = HashMap::new();
    let mut cost = None;

    while let Some((cursor, heading, score, mut path)) = agenda.pop() {
        println!("{}", agenda.len());

        if path.contains_key(&(cursor, heading)) {
            continue;
        }

        if cache
            .get(&(cursor, heading))
            .is_some_and(|tally| *tally < score)
        {
            continue;
        }

        cache.insert((cursor, heading), score);
        path.insert((cursor, heading), score);

        if cursor == end {
            if cost == None {
                cost = Some(score);
            }
            valid_paths.extend(path.iter());
            continue;
        }

        let step = heading.step(&cursor);

        if !map[step.1][step.0] {
            agenda.push((step, heading, score + 1, path.clone()));
        }

        let left_rotate = heading.rotate_left();
        let right_rotate = heading.rotate_right();

        if !map[left_rotate.step(&cursor).1][left_rotate.step(&cursor).0] {
            agenda.push((cursor, left_rotate, score + 1000, path.clone()));
        }
        if !map[right_rotate.step(&cursor).1][right_rotate.step(&cursor).0] {
            agenda.push((cursor, right_rotate, score + 1000, path.clone()));
        }

        if let Some(cost) = cost {
            agenda = agenda.into_iter().filter(|candidate| {
                candidate.2 <= cost || !candidate.3.iter().any(|(key, tally)| {
                    valid_paths.get(key).is_some_and(|ideal| ideal < tally)
                })
            }).collect();
        }

        agenda.sort_by(|a, b| b.2.cmp(&a.2));
    }

    if let Some(found_cost) = cost {
        (found_cost, valid_paths.into_iter().map(|((cursor, _heading), _score)| cursor).collect::<HashSet<_>>().len())
    } else {
        panic!("Agenda exhausted with no solution")
    }

}

fn cheapest_route_cost() -> (usize, usize) {
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
    let (part_one_answer, part_two_answer) = cheapest_route_cost();
    println!("part one: {}", part_one_answer);
    println!("part two: {}", part_two_answer);
}
