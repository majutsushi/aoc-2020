use std::fs;

use anyhow::{Context, Result};
use itertools::process_results;
use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
use proptest::prelude::*;

lazy_static! {
    static ref PW_LINE: Regex =
        Regex::new(r"^(?P<num1>\w+)-(?P<num2>\d+) (?P<char>[a-z]): (?P<pw>\w+)$").unwrap();
}

struct PwInfo {
    num1: usize,
    num2: usize,
    char: char,
    pw: String,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/02.txt").expect("Error reading input file");

    let count1 = get_count(is_valid_pw_part1, &input)?;
    println!("Part 1: {}", count1);

    let count2 = get_count(is_valid_pw_part2, &input)?;
    println!("Part 2: {}", count2);

    Ok(())
}

fn get_count<F>(f: F, input: &str) -> Result<usize>
where
    F: Fn(&str) -> Result<bool>,
{
    let valid_results = input.lines().map(|line| f(line));
    process_results(valid_results, |iter| iter.filter(|&b| b).count())
}

fn parse_line(line: &str) -> Result<PwInfo> {
    let caps = PW_LINE
        .captures(line)
        .with_context(|| format!("Failed to match line: {}", line))?;

    let num1 = caps.name("num1").unwrap().as_str().parse::<usize>()?;
    let num2 = caps.name("num2").unwrap().as_str().parse::<usize>()?;
    let char = caps.name("char").unwrap().as_str().parse::<char>()?;
    let pw = caps.name("pw").unwrap().as_str().to_string();

    Ok(PwInfo {
        num1,
        num2,
        char,
        pw,
    })
}

fn is_valid_pw_part1(line: &str) -> Result<bool> {
    let pw_info = parse_line(line)?;

    let char_re = Regex::new(&format!(r"{}+", pw_info.char))?;
    let num_chars = char_re
        .find_iter(&pw_info.pw)
        .map(|m| m.as_str().len())
        .sum();

    Ok(pw_info.num1 <= num_chars && num_chars <= pw_info.num2)
}

fn is_valid_pw_part2(line: &str) -> Result<bool> {
    let pw_info = parse_line(line)?;
    let pw_vec = pw_info.pw.chars().collect::<Vec<_>>();

    let pos1_match = pw_vec[pw_info.num1 - 1] == pw_info.char;
    let pos2_match = pw_vec[pw_info.num2 - 1] == pw_info.char;
    let is_valid = pos1_match ^ pos2_match;

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_part1_examples() {
        assert!(is_valid_pw_part1("1-3 a: abcde").unwrap());
        assert!(!is_valid_pw_part1("1-3 b: cdefg").unwrap());
        assert!(is_valid_pw_part1("2-9 c: ccccccccc").unwrap());
    }

    #[test]
    fn test_validation_part1_complex() {
        assert!(is_valid_pw_part1("1-3 a: ababa").unwrap());
        assert!(!is_valid_pw_part1("1-3 a: abababa").unwrap());
        assert!(is_valid_pw_part1("1-3 a: abaab").unwrap());
    }

    #[test]
    fn test_validation_part2_examples() {
        assert!(is_valid_pw_part2("1-3 a: abcde").unwrap());
        assert!(!is_valid_pw_part2("1-3 b: cdefg").unwrap());
        assert!(!is_valid_pw_part2("2-9 c: ccccccccc").unwrap());
    }

    proptest! {
        #[test]
        fn doesnt_crash(s in r"\PC*") {
            is_valid_pw_part1(&s).ok();
            is_valid_pw_part2(&s).ok();
        }
    }
}
