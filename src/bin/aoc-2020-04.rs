use std::collections::{HashMap, LinkedList};
use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

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
}
impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = HashMap::new();
        let mut errors = LinkedList::new();
        for field in s.split_whitespace() {
            if let Some((key, val)) = field.splitn(2, ':').collect_tuple() {
                entries.insert(key.to_string(), val.to_string());
            } else {
                errors.push_back(field);
            }
        }
        if !errors.is_empty() {
            return Err(anyhow!(
                "No colon found in fields: {}",
                errors.into_iter().join(", ")
            ));
        }

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

    let data = input
        .split("\n\n")
        .map(|s| s.parse::<Passport>())
        .collect::<Vec<_>>();

    let errors = data.iter().filter(|r| r.is_err()).collect::<Vec<_>>();
    if !errors.is_empty() {
        println!("Parse errors: {:?}", errors);
    }

    let count = data
        .iter()
        .filter(|r| r.is_ok())
        .map(|r| r.as_ref().unwrap().is_valid())
        .filter(|&p| p)
        .count();

    println!("{}", count);

    Ok(())
}
