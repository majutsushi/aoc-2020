use std::cmp::{max, min, Ordering};

use anyhow::{Context, Result};

#[derive(Debug, Copy, Clone)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}
impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Floor,
            'L' => Tile::Empty,
            '#' => Tile::Occupied,
            _ => panic!("Unrecognized character: {}", c),
        }
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/11.txt").context("Error reading input file")?;

    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect::<Vec<Tile>>())
        .collect::<Vec<_>>();

    println!("Part 1: {}", compute_grid(&grid, near_part1, 3));
    println!("Part 2: {}", compute_grid(&grid, near_part2, 4));

    Ok(())
}

fn compute_grid<F>(grid: &[Vec<Tile>], near_func: F, max_occupied: usize) -> usize
where
    F: Fn(&[Vec<Tile>], i8, i8, i8, i8) -> usize,
{
    let mut grid = grid.to_owned();

    let width = grid[0].len() as i8;
    let height = grid.len() as i8;

    loop {
        let mut changed = false;
        let mut new_grid = grid.clone();
        for row in 0..height {
            for col in 0..width {
                let row = row as usize;
                let col = col as usize;

                // Don't compute the visible seats for floor tiles
                let num_occupied = match grid[row][col] {
                    Tile::Floor => 0,
                    _ => near_func(&grid, row as i8, col as i8, height, width),
                };

                new_grid[row][col] = match grid[row][col] {
                    Tile::Empty if num_occupied == 0 => {
                        changed = true;
                        Tile::Occupied
                    }
                    Tile::Occupied if num_occupied > max_occupied => {
                        changed = true;
                        Tile::Empty
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

    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|col| matches!(col, Tile::Occupied))
                .count()
        })
        .sum()
}

fn near_part1(grid: &[Vec<Tile>], row: i8, col: i8, height: i8, width: i8) -> usize {
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

    num_occupied
}

fn near_part2(grid: &[Vec<Tile>], row: i8, col: i8, _height: i8, _width: i8) -> usize {
    let mut num_occupied = 0;

    for (step_x, step_y) in &[
        (1, 1),
        (1, 0),
        (1, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, 1),
        (0, -1),
    ] {
        if occupied_seat_visible(&grid, row, col, *step_x, *step_y) {
            num_occupied += 1;
        }
    }

    num_occupied
}

fn occupied_seat_visible(grid: &[Vec<Tile>], row: i8, col: i8, step_x: i8, step_y: i8) -> bool {
    let range_x = match step_x.cmp(&0i8) {
        Ordering::Less => (0..row).rev().collect::<Vec<_>>(),
        Ordering::Equal => vec![row; grid[0].len()],
        Ordering::Greater => ((row + 1)..grid.len() as i8).collect::<Vec<_>>(),
    };
    let range_y = match step_y.cmp(&0i8) {
        Ordering::Less => (0..col).rev().collect::<Vec<_>>(),
        Ordering::Equal => vec![col; grid.len()],
        Ordering::Greater => ((col + 1)..grid[0].len() as i8).collect::<Vec<_>>(),
    };

    for (&x, y) in range_x.iter().zip(range_y) {
        match grid[x as usize][y as usize] {
            Tile::Floor => continue,
            Tile::Empty => return false,
            Tile::Occupied => return true,
        }
    }

    false
}
