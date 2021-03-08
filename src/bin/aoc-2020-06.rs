use std::collections::HashSet;
use std::fs;

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/06.txt").context("Error reading input file")?;

    let part1: usize = input
        .split("\n\n")
        .map(|block| block.chars().filter(|c| c.is_alphabetic()).unique().count())
        .sum();
    println!("Part 1: {}", part1);

    let part2: usize = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold1(|x, y| x.intersection(&y).cloned().collect::<HashSet<_>>())
                .unwrap()
                .len()
        })
        .sum();
    println!("Part 2: {}", part2);

    Ok(())
}
