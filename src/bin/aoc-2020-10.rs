use std::collections::HashMap;

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

    let mut cache = HashMap::new();
    let sum = count_combinations(&adapters, &mut cache);
    println!("Part 2: {}", sum);

    Ok(())
}

fn count_combinations<'a>(slice: &'a [u8], cache: &mut HashMap<&'a [u8], u64>) -> u64 {
    if slice.len() <= 2 {
        return 1;
    }

    let mut sum = 0;
    for i in 1.. {
        if i >= slice.len() || slice[i] - slice[0] > 3 {
            break;
        }

        sum += match cache.get(&slice[i..]) {
            Some(&v) => v,
            None => count_combinations(&slice[i..], cache),
        }
    }

    (*cache).insert(&slice, sum);
    sum
}
