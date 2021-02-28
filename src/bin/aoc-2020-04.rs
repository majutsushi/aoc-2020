use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::{Either, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CM_VALUE: Regex = Regex::new(r"^(?P<val>\d+)cm$").unwrap();
    static ref IN_VALUE: Regex = Regex::new(r"^(?P<val>\d+)in$").unwrap();
    static ref HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn is_valid(&self) -> bool {
        matches!(
            self,
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _
            }
        )
    }

    fn is_valid_part2(&self) -> bool {
        fn fields_valid(p: &Passport) -> Result<bool> {
            let byr = p.byr.as_ref().unwrap().parse::<u16>()?;
            if !(1920..=2002).contains(&byr) {
                return Ok(false);
            }

            let iyr = p.iyr.as_ref().unwrap().parse::<u16>()?;
            if !(2010..=2020).contains(&iyr) {
                return Ok(false);
            }

            let eyr = p.eyr.as_ref().unwrap().parse::<u16>()?;
            if !(2020..=2030).contains(&eyr) {
                return Ok(false);
            }

            let hgt = p.hgt.as_ref().unwrap();
            if let Some(captures) = CM_VALUE.captures(hgt) {
                let val = captures.name("val").unwrap().as_str().parse::<u16>()?;
                if !(150..=193).contains(&val) {
                    return Ok(false);
                }
            } else if let Some(captures) = IN_VALUE.captures(hgt) {
                let val = captures.name("val").unwrap().as_str().parse::<u16>()?;
                if !(59..=76).contains(&val) {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }

            if !HCL.is_match(p.hcl.as_ref().unwrap()) {
                return Ok(false);
            }

            match p.ecl.as_ref().unwrap().as_ref() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
                _ => return Ok(false),
            }

            if !PID.is_match(p.pid.as_ref().unwrap()) {
                return Ok(false);
            }

            Ok(true)
        }
        self.is_valid() && fields_valid(&self).unwrap_or(false)
    }
}
impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (parsed_fields, errors): (Vec<_>, Vec<_>) = s
            .split_whitespace()
            .map(|field| {
                field
                    .splitn(2, ':')
                    .map(|p| p.to_owned())
                    .collect_tuple()
                    .ok_or(field)
            })
            .partition_map(|r| match r {
                Ok(t) => Either::Left(t),
                Err(e) => Either::Right(e),
            });

        if !errors.is_empty() {
            return Err(anyhow!(
                "No colon found in fields: {}",
                errors.into_iter().join(", ")
            ));
        }

        let mut entries = parsed_fields
            .into_iter()
            .collect::<HashMap<String, String>>();

        let result = Passport {
            byr: entries.remove("byr"),
            iyr: entries.remove("iyr"),
            eyr: entries.remove("eyr"),
            hgt: entries.remove("hgt"),
            hcl: entries.remove("hcl"),
            ecl: entries.remove("ecl"),
            pid: entries.remove("pid"),
            cid: entries.remove("cid"),
        };
        if entries.is_empty() {
            Ok(result)
        } else {
            Err(anyhow!("Unknown fields found: {:?}", entries))
        }
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input/04.txt").context("input file not found")?;

    let (passports, errors): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|s| s.parse::<Passport>())
        .partition_map(|r| match r {
            Ok(p) => Either::Left(p),
            Err(e) => Either::Right(e),
        });

    if !errors.is_empty() {
        println!("Parse errors: {:?}", errors);
    }

    let count = passports
        .iter()
        .map(|p| p.is_valid())
        .filter(|&p| p)
        .count();
    println!("Part 1: {}", count);

    let count2 = passports
        .iter()
        .map(|r| r.is_valid_part2())
        .filter(|&p| p)
        .count();
    println!("Part 2: {}", count2);

    Ok(())
}
