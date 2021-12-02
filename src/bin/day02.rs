use std::{error::Error, fs, str::FromStr};

fn main() {
    let raw = fs::read_to_string("assets/day02.txt").expect("could not open file!");
    let instructions: Vec<Instruction> = raw.lines().map(|l| l.parse().unwrap()).collect();

    dbg!(part_a(&instructions));
    dbg!(part_b(&instructions));
}

fn part_a(instructions: &[Instruction]) -> i32 {
    let mut h_pos = 0i32;
    let mut depth = 0i32;

    for instr in instructions {
        match instr {
            Instruction::Forward(d) => h_pos += d,
            Instruction::Down(d) => depth += d,
            Instruction::Up(d) => depth -= d,
        }
    }
    h_pos * depth
}

fn part_b(instructions: &[Instruction]) -> i32 {
    let mut h_pos = 0i32;
    let mut depth = 0i32;
    let mut aim = 0i32;

    for instr in instructions {
        match instr {
            Instruction::Forward(d) => {
                h_pos += d;
                depth += aim * d;
            }
            Instruction::Down(d) => aim += d,
            Instruction::Up(d) => aim -= d,
        }
    }
    h_pos * depth
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

// Let's try to implement the FromStr trait to be able to use `parse()`
impl FromStr for Instruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Expected instruction in the form '<instr> <d>'".into());
        };
        let n: i32 = parts[1].parse()?;
        match parts[0] {
            "forward" => Ok(Instruction::Forward(n)),
            "down" => Ok(Instruction::Down(n)),
            "up" => Ok(Instruction::Up(n)),
            _ => Err(format!("Unknown instruction '{}'", &parts[0]).into()),
        }
    }
}
