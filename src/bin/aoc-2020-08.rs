use std::collections::HashSet;
use std::fs;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSTRUCTION: Regex = Regex::new(r"^(?P<opcode>\w+) (?P<operand>[+-]\d+)$").unwrap();
}

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i16),
    Jmp(i16),
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/08.txt").context("Error reading input file")?;

    let mut instructions = Vec::new();
    for line in input.lines() {
        let caps = INSTRUCTION
            .captures(line)
            .with_context(|| format!("Failed to match line: {}", line))?;

        let operand = caps.name("operand").unwrap().as_str().parse::<i16>()?;
        let instruction = match caps.name("opcode").unwrap().as_str() {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            _ => return Err(anyhow!("Unknown instruction: {}", line)),
        };

        instructions.push(instruction);
    }

    let mut ip: i16 = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();
    while !seen.contains(&ip) {
        seen.insert(ip);
        match instructions.get(ip as usize).unwrap() {
            Instruction::Nop => ip += 1,
            Instruction::Acc(v) => {
                acc += v;
                ip += 1
            }
            Instruction::Jmp(v) => ip += v,
        };
    }

    println!("{}", acc);

    Ok(())
}
