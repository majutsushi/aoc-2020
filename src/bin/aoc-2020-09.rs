use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/09.txt").context("Error reading input file")?;

    let numbers = input
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .with_context(|| format!("Error parsing line: {}", line))
        })
        .collect::<Result<Vec<_>>>()?;

    let invalid_num = part1(&numbers)?;
    println!("Part 1: {}", invalid_num);

    let weakness = part2(&numbers, invalid_num);
    println!("Part 2: {}", weakness);

    Ok(())
}

fn part1(numbers: &[usize]) -> Result<usize> {
    for (i, &num) in numbers.iter().enumerate().skip(25) {
        let valid = &numbers[i - 25..i]
            .iter()
            .combinations(2)
            .map(|c| c.iter().copied().sum())
            .any(|s: usize| s == num);

        if !valid {
            return Ok(num);
        }
    }

    Err(anyhow!("No invalid number found"))
}

fn part2(numbers: &[usize], invalid_num: usize) -> usize {
    let mut lower = 0;
    let mut upper = 1;
    let mut total = numbers[lower] + numbers[upper];

    while total != invalid_num {
        while total < invalid_num {
            upper += 1;
            total += numbers[upper];
        }
        while total > invalid_num {
            total -= numbers[lower];
            lower += 1;
        }
    }

    let slice = &numbers[lower..=upper];
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}
