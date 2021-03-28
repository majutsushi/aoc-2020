use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, operand) = s.splitn(2, ' ').collect_tuple().unwrap();
        let operand = operand.parse::<i16>()?;

        let instruction = match opcode {
            "nop" => Instruction::Nop(operand),
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            _ => return Err(anyhow!("Unknown instruction: {}", s)),
        };
        Ok(instruction)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/08.txt").context("Error reading input file")?;

    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>())
        .collect::<Result<Vec<_>>>()?;

    match run_instructions(&instructions) {
        (true, _) => return Err(anyhow!("Program finished unexpectedly")),
        (false, acc) => println!("Part 1: {}", acc),
    }

    for i in 0..instructions.len() {
        let mut cur_instructions = instructions.clone();
        if let Some(instruction) = cur_instructions.get_mut(i) {
            match instruction {
                Instruction::Nop(op) => *instruction = Instruction::Jmp(*op),
                Instruction::Jmp(op) => *instruction = Instruction::Nop(*op),
                _ => continue,
            }
        }

        if let (true, acc) = run_instructions(&cur_instructions) {
            println!("Part 2: {}", acc);
            break;
        }
    }

    Ok(())
}

fn run_instructions(instructions: &[Instruction]) -> (bool, i16) {
    let mut ip: i16 = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&ip) {
        seen.insert(ip);

        if ip as usize >= instructions.len() {
            return (true, acc);
        }

        match instructions.get(ip as usize).unwrap() {
            Instruction::Nop(_) => ip += 1,
            Instruction::Acc(op) => {
                acc += op;
                ip += 1
            }
            Instruction::Jmp(op) => ip += op,
        };
    }

    (false, acc)
}
