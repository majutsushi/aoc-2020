use std::fs;
use std::str::FromStr;

use anyhow::{Context, Result};
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
        fn lower((min, max): (u8, u8)) -> (u8, u8) {
            (min, (max + min + 1) / 2 - 1)
        }
        fn higher((min, max): (u8, u8)) -> (u8, u8) {
            ((max + min + 1) / 2, max)
        }

        let (rows, columns) = s.split_at(7);

        let (row_min, row_max) = rows
            .chars()
            .map(|c| match c {
                'F' => lower,
                'B' => higher,
                _ => panic!("Invalid character: {}", c),
            })
            .fold((0, 127), |acc, f| f(acc));
        assert!(row_min == row_max);

        let (col_min, col_max) = columns
            .chars()
            .map(|c| match c {
                'L' => lower,
                'R' => higher,
                _ => panic!("Invalid character: {}", c),
            })
            .fold((0, 7), |acc, f| f(acc));
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

    for (a, b) in seats.iter().tuple_windows::<(_, _)>() {
        if b.id != a.id + 1 {
            println!("Part 2: {}", a.id + 1);
            break;
        }
    }

    Ok(())
}
