use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::{Either, Itertools};

#[derive(Debug)]
struct Seat {
    row: u8,
    column: u8,
    id: u16,
}
impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rows, columns) = s.split_at(7);

        let mut row_min = 0;
        let mut row_max = 127;
        for c in rows.chars() {
            match c {
                'F' => row_max = (row_max + row_min + 1) / 2 - 1,
                'B' => row_min = (row_max + row_min + 1) / 2,
                _ => return Err(anyhow!("Invalid row character: {}", c)),
            }
        }
        assert!(row_min == row_max);

        let mut col_min = 0;
        let mut col_max = 7;
        for c in columns.chars() {
            match c {
                'L' => col_max = (col_max + col_min + 1) / 2 - 1,
                'R' => col_min = (col_max + col_min + 1) / 2,
                _ => return Err(anyhow!("Invalid column character: {}", c)),
            }
        }
        assert!(col_min == col_max);

        Ok(Seat {
            row: row_min,
            column: col_min,
            id: row_min as u16 * 8 + col_min as u16,
        })
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/05.txt").context("Error reading input file")?;

    let (mut seats, errors): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.parse::<Seat>())
        .partition_map(|r| match r {
            Ok(p) => Either::Left(p),
            Err(e) => Either::Right(e),
        });
    if !errors.is_empty() {
        println!("Parse errors: {:?}", errors);
    }

    seats.sort_by(|a, b| a.id.cmp(&b.id));
    println!("Part 1: {}", seats.last().unwrap().id);

    Ok(())
}
