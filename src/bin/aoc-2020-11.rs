use std::cmp::{max, min};

use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Floor,
    Emtpy,
    Occupied,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Floor,
            'L' => Tile::Emtpy,
            '#' => Tile::Occupied,
            _ => panic!("Unrecognized character: {}", c),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/11.txt").context("Error reading input file")?;

    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();

    let width = grid[0].len() as i8;
    let height = grid.len() as i8;

    loop {
        let mut changed = false;
        let mut new_grid = grid.clone();
        for row in 0..height {
            for col in 0..width {
                let mut num_occupied = 0;
                for x in max(row - 1, 0)..=min(row + 1, height - 1) {
                    for y in max(col - 1, 0)..=min(col + 1, width - 1) {
                        if x == row && col == y {
                            continue; // Don't count the current seat
                        }
                        if matches!(grid[x as usize][y as usize], Tile::Occupied) {
                            num_occupied += 1;
                        }
                    }
                }
                let row = row as usize;
                let col = col as usize;
                new_grid[row][col] = match grid[row][col] {
                    Tile::Emtpy if num_occupied == 0 => {
                        changed = true;
                        Tile::Occupied
                    }
                    Tile::Occupied if num_occupied >= 4 => {
                        changed = true;
                        Tile::Emtpy
                    }
                    _ => grid[row][col],
                };
            }
        }
        grid = new_grid;

        if !changed {
            break;
        }
    }

    let num_occupied: usize = grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|col| matches!(col, Tile::Occupied))
                .count()
        })
        .sum();

    println!("{}", num_occupied);

    Ok(())
}
