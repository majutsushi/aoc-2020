use std::collections::HashSet;
use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION: Regex = Regex::new(r"^(?P<opcode>\w+) (?P<operand>[+-]\d+)$").unwrap();
}

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i16),
    Acc(i16),
    Jmp(i16),
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = INSTRUCTION
            .captures(s)
            .with_context(|| format!("Failed to match instruction: {}", s))?;

        let operand = caps.name("operand").unwrap().as_str().parse::<i16>()?;
        let instruction = match caps.name("opcode").unwrap().as_str() {
            "nop" => Instruction::Nop(operand),
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            _ => return Err(anyhow!("Unknown instruction: {}", s)),
        };
        Ok(instruction)
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/08.txt").context("Error reading input file")?;

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
