use std::fs;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref PW_LINE: Regex =
        Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z]): (?P<pw>\w+)$").unwrap();
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/02").expect("Error reading file");
    let count = input
        .lines()
        .map(|line| is_valid_pw(&line).unwrap())
        .filter(|b| *b)
        .count();
    println!("{}", count);
    Ok(())
}

fn is_valid_pw(line: &str) -> Result<bool> {
    let caps = PW_LINE
        .captures(line)
        .with_context(|| format!("Failed to match line: {}", line))?;

    let min = caps.name("min").unwrap().as_str().parse::<usize>().unwrap();
    let max = caps.name("max").unwrap().as_str().parse::<usize>().unwrap();
    let char = caps.name("char").unwrap().as_str();
    let pw = caps.name("pw").unwrap().as_str();

    let char_re = Regex::new(&format!(r"{}+", char)).unwrap();
    let num_chars = char_re
        .find_iter(pw)
        .map(|m| m.as_str().len())
        .fold(0, |acc, x| acc + x);

    Ok(min <= num_chars && num_chars <= max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_examples() {
        assert!(is_valid_pw("1-3 a: abcde").unwrap());
        assert!(!is_valid_pw("1-3 b: cdefg").unwrap());
        assert!(is_valid_pw("2-9 c: ccccccccc").unwrap());
    }

    #[test]
    fn test_validation_complex() {
        assert!(is_valid_pw("1-3 a: ababa").unwrap());
        assert!(!is_valid_pw("1-3 a: abababa").unwrap());
        assert!(is_valid_pw("1-3 a: abaab").unwrap());
    }
}
