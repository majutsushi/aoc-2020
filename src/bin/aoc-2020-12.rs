use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
enum Instruction {
    North(i16),
    South(i16),
    East(i16),
    West(i16),
    Left(i16),
    Right(i16),
    Forward(i16),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[1..].parse::<i16>()?;
        let instruction = match &s[0..1] {
            "N" => Instruction::North(arg),
            "S" => Instruction::South(arg),
            "E" => Instruction::East(arg),
            "W" => Instruction::West(arg),
            "L" => Instruction::Left(arg),
            "R" => Instruction::Right(arg),
            "F" => Instruction::Forward(arg),
            _ => return Err(anyhow!("Unrecognized instruction: {}", s)),
        };
        Ok(instruction)
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}
impl Facing {
    fn turn(&self, degrees: i16) -> Self {
        if !matches!(degrees.abs(), 90 | 180 | 270) {
            panic!("Unsupported turn degrees: {}", degrees)
        }
        let v = vec![Self::North, Self::East, Self::South, Self::West];
        let self_index = v.iter().position(|f| f == self).unwrap();
        let mut new_index = (self_index as i16 + degrees / 90) % v.len() as i16;
        if new_index < 0 {
            new_index += v.len() as i16;
        }
        v[new_index as usize]
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt").context("Error reading input file")?;

    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<_>>>()?;

    let mut position = Position { x: 0, y: 0 };
    let mut facing = Facing::East;
    for instruction in instructions {
        match instruction {
            Instruction::North(a) => position.y += a,
            Instruction::South(a) => position.y -= a,
            Instruction::East(a) => position.x += a,
            Instruction::West(a) => position.x -= a,
            Instruction::Left(a) => facing = facing.turn(-a),
            Instruction::Right(a) => facing = facing.turn(a),
            Instruction::Forward(a) => match facing {
                Facing::North => position.y += a,
                Facing::South => position.y -= a,
                Facing::East => position.x += a,
                Facing::West => position.x -= a,
            },
        }
    }

    println!("{}", position.x.abs() + position.y.abs());

    Ok(())
}
