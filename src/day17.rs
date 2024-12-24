use std::fs;

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    out_store: Vec<usize>,
    pointer: usize
}

impl Computer {
    fn from(input: &str) -> Option<Self> {
        match input
            .lines()
            .flat_map(|line| line.split_whitespace())
            .filter_map(|word| word.parse().ok())
            .collect::<Vec<usize>>()[..]
        {
            [a, b, c] => Some(Self {
                a, b, c,
                out_store: vec![],
                pointer: 0
            }),
            _ => None
        }
    }

    fn combo(&self, parameter: usize) -> usize {
        match parameter {
            0 | 1 | 2 | 3 => parameter,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved value!"),
            _ => panic!("Invalid parameter")
        }
    }

    fn adv(&mut self, parameter: usize) {
        self.a = self.a / 2_usize.pow(self.combo(parameter) as u32);
    }

    fn bxl(&mut self, parameter: usize) {
        self.b = self.b ^ parameter;
    }

    fn bst(&mut self, parameter: usize) {
        self.b = self.combo(parameter) % 8;
    }

    fn jnz(&mut self, parameter: usize) {
        match self.a {
            0 => {},
            _ => self.pointer = parameter
        };
    }

    fn bxc(&mut self, _parameter: usize) {
        self.b = self.b ^ self.c;
    }

    fn out(&mut self, parameter: usize) {
        self.out_store.push(self.combo(parameter) % 8);
    }

    fn bdv(&mut self, parameter: usize) {
        self.b = self.a / 2_usize.pow(self.combo(parameter) as u32);
    }

    fn cdv(&mut self, parameter: usize) {
        self.c = self.a / 2_usize.pow(self.combo(parameter) as u32);
    }
}

fn out_values() -> String {
    if let Some(input) = fs::read_to_string("data/17.input").ok() {
        if let Some((registers, instructions)) = input.split_once("\n\n") {
            let mut computer = Computer::from(registers).unwrap();
            let instructions = instructions
                .replace("Program: ", "")
                .trim()
                .split(",")
                .filter_map(|op| op.parse().ok())
                .collect::<Vec<usize>>();

            while let Some((instruction, parameter)) = instructions
                .iter()
                .nth(computer.pointer)
                .zip(instructions.iter().nth(computer.pointer + 1))
            {
                computer.pointer = computer.pointer + 2;

                match instruction {
                    0 => computer.adv(*parameter),
                    1 => computer.bxl(*parameter),
                    2 => computer.bst(*parameter),
                    3 => computer.jnz(*parameter),
                    4 => computer.bxc(*parameter),
                    5 => computer.out(*parameter),
                    6 => computer.bdv(*parameter),
                    7 => computer.cdv(*parameter),
                    _ => {}
                }
            }

            computer
                .out_store
                .into_iter()
                .map(|value| value.to_string())
                .collect::<Vec<String>>()
                .join(",")
        } else {
            panic!("Input isn't split into computer state and instructions")
        }
    } else {
        panic!("No puzzle input")
    }
}

fn main() {
    println!("part one: {}", out_values());
}

