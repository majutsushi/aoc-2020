use std::fs;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/03").context("input file not found")?;

    let map = input
        .lines()
        .map(|line| {
            line.split("")
                // splitting on every character produces
                // empty strings at beginning and end
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = map[0].len();
    let count = (0..map.len())
        .zip((0..).step_by(3))
        .filter(|&(y, x)| map[y][x % width] == "#")
        .count();
    println!("{}", count);

    Ok(())
}
