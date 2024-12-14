use std::fs;

struct Game {
    a_increment: (usize, usize),
    b_increment: (usize, usize),
    target: (usize, usize)
}

fn parse_game_states(input: String) -> Vec<Game> {
    input
        .split("\n\n")
        .filter_map(|game| {
            if game.lines().count() != 3 {
                return None;
            }

            let a_line = game
                .lines()
                .nth(0)
                .unwrap()
                .replace("Button A: X+", "")
                .replace("Y+", "");
            let b_line = game
                .lines()
                .nth(1)
                .unwrap()
                .replace("Button B: X+", "")
                .replace("Y+", "");
            let target_line = game
                .lines()
                .nth(2)
                .unwrap()
                .replace("Prize: X=", "")
                .replace("Y=", "");

            if let Some((ax, ay)) = a_line.split_once(", ") {
                if let Some((bx, by)) = b_line.split_once(", ") {
                    if let Some((tx, ty)) = target_line.split_once(", ") {
                        let ax = ax.parse::<usize>().ok();
                        let ay = ay.parse::<usize>().ok();
                        let bx = bx.parse::<usize>().ok();
                        let by = by.parse::<usize>().ok();
                        let tx = tx.parse::<usize>().ok();
                        let ty = ty.parse::<usize>().ok();

                        if [ax, ay, bx, by, tx, ty].iter().all(|num| num.is_some()) {
                            return Some(Game {
                                a_increment: (ax.unwrap(), ay.unwrap()),
                                b_increment: (bx.unwrap(), by.unwrap()),
                                target:      (tx.unwrap(), ty.unwrap()),
                            });
                        }
                    }
                }
            }

            None
        })
        .collect()
}

fn cheapest_solution(game: &Game) -> Option<usize> {
    (0..=100).flat_map(|a_presses| {
        (0..=100).filter_map(|b_presses| {
            if (a_presses * game.a_increment.0 + b_presses * game.b_increment.0) == game.target.0 &&
                (a_presses * game.a_increment.1 + b_presses * game.b_increment.1) == game.target.1
            {
                Some(a_presses * 3 + b_presses)
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).min()
}

fn foo() -> usize {
    if let Some(input) = fs::read_to_string("data/13.input").ok() {
        parse_game_states(input)
            .iter()
            .filter_map(|game_state| cheapest_solution(game_state))
            .sum()
    } else {
        panic!("No puzzle input");
    }
}

fn main() {
    println!("part one: {}", foo());
}
