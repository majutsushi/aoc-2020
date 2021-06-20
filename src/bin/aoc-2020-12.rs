use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s[1..].parse::<i32>()?;
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
    x: i32,
    y: i32,
}
impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn rotate(&mut self, degrees: i32) -> Result<()> {
        *self = match degrees {
            90 | -270 => Position::new(self.y, -self.x),
            180 | -180 => Position::new(-self.x, -self.y),
            270 | -90 => Position::new(-self.y, self.x),
            _ => return Err(anyhow!("Unsupported rotation degrees: {}", degrees)),
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}
impl Facing {
    fn turn(&mut self, degrees: i32) -> Result<()> {
        let degrees = match degrees {
            -90 => 270,
            -180 => 180,
            -270 => 90,
            d @ (90 | 180 | 270) => d,
            _ => return Err(anyhow!("Unsupported turn degrees: {}", degrees)),
        };
        let facings = vec![Self::North, Self::East, Self::South, Self::West];
        let self_index = facings.iter().position(|f| f == self).unwrap() as i32;
        let new_index = (self_index + degrees / 90) % facings.len() as i32;

        *self = facings[new_index as usize];
        Ok(())
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/12.txt").context("Error reading input file")?;

    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<_>>>()?;

    println!("Part 1: {}", part1(&instructions)?);
    println!("Part 2: {}", part2(&instructions)?);

    Ok(())
}

fn part1(instructions: &[Instruction]) -> Result<i32> {
    let mut position = Position::new(0, 0);
    let mut facing = Facing::East;
    for instruction in instructions {
        match *instruction {
            Instruction::North(a) => position.y += a,
            Instruction::South(a) => position.y -= a,
            Instruction::East(a) => position.x += a,
            Instruction::West(a) => position.x -= a,
            Instruction::Left(a) => facing.turn(-a)?,
            Instruction::Right(a) => facing.turn(a)?,
            Instruction::Forward(a) => match facing {
                Facing::North => position.y += a,
                Facing::South => position.y -= a,
                Facing::East => position.x += a,
                Facing::West => position.x -= a,
            },
        }
    }

    Ok(position.x.abs() + position.y.abs())
}

fn part2(instructions: &[Instruction]) -> Result<i32> {
    let mut ship = Position::new(0, 0);
    let mut waypoint = Position::new(10, 1);

    for instruction in instructions {
        match *instruction {
            Instruction::North(a) => waypoint.y += a,
            Instruction::South(a) => waypoint.y -= a,
            Instruction::East(a) => waypoint.x += a,
            Instruction::West(a) => waypoint.x -= a,
            Instruction::Left(a) => waypoint.rotate(-a)?,
            Instruction::Right(a) => waypoint.rotate(a)?,
            Instruction::Forward(a) => {
                ship.y += a * waypoint.y;
                ship.x += a * waypoint.x;
            }
        }
    }

    Ok(ship.x.abs() + ship.y.abs())
}
