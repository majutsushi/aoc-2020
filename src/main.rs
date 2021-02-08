use std::fs;

fn main() {
    let input = fs::read_to_string("input".to_string()).expect("Error reading file");
    let mut lines = input
        .lines()
        .map(|s| {
            s.parse::<u32>()
                .unwrap_or_else(|_| panic!("failed to parse {}", &s))
        })
        .collect::<Vec<_>>();
    lines.sort();

    match part_one(&lines) {
        Ok(m) => println!("✔ Part one: {}", m),
        Err(msg) => println!("❌ Part one: {}", msg),
    }
    match part_two(&lines) {
        Ok(m) => println!("✔ Part two: {}", m),
        Err(msg) => println!("❌ Part two: {}", msg),
    }
}

fn part_one(lines: &[u32]) -> Result<u32, String> {
    for a in &lines[..lines.len() - 1] {
        for b in &lines[..lines.len()] {
            if a + b == 2020 {
                return Ok(a * b);
            } else if a + b > 2020 {
                break;
            }
        }
    }

    Err("No result found".to_string())
}

fn part_two(lines: &[u32]) -> Result<u32, String> {
    for a in &lines[..lines.len() - 2] {
        for b in &lines[..lines.len() - 1] {
            for c in &lines[..lines.len()] {
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                } else if a + b + c > 2020 {
                    break;
                }
            }
        }
    }

    Err("No result found".to_string())
}
