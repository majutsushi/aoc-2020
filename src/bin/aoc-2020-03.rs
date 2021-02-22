use std::fs;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = fs::read_to_string("input/03").context("input file not found")?;

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", get_tree_count(&map, 3, 1));

    let part2 = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
        .iter()
        .map(|&[x, y]| get_tree_count(&map, x, y))
        .product::<usize>();
    println!("Part 2: {}", part2);

    Ok(())
}

fn get_tree_count(map: &[Vec<char>], step_x: usize, step_y: usize) -> usize {
    let width = map[0].len();
    (0..map.len())
        .step_by(step_y)
        .zip((0..).step_by(step_x))
        .filter(|&(y, x)| map[y][x % width] == '#')
        .count()
}
