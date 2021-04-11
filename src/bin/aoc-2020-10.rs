use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/10.txt").context("Error reading input file")?;

    let mut adapters = input
        .lines()
        .map(|line| {
            line.parse::<u8>()
                .with_context(|| format!("Error parsing line {}", line))
        })
        .collect::<Result<Vec<_>>>()?;
    adapters.sort_unstable();

    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);
    let (ones, threes): (Vec<_>, Vec<_>) = adapters
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .partition(|&i| i == 1);
    println!("Part 1: {}", ones.len() * threes.len());

    Ok(())
}
