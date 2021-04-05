use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/09.txt").context("Error reading input file")?;

    let numbers = input
        .lines()
        .map(|line| {
            line.parse::<usize>()
                .with_context(|| format!("Error parsing line: {}", line))
        })
        .collect::<Result<Vec<_>>>()?;

    for (i, num) in numbers.iter().enumerate().skip(25) {
        let mut valid = false;
        'outer: for (j, j_num) in numbers[i - 25..i - 1].iter().enumerate() {
            for k_num in &numbers[j + 1..i] {
                if j_num + k_num == *num {
                    valid = true;
                    break 'outer;
                }
            }
        }

        if !valid {
            println!("{}", num);
            break;
        }
    }

    Ok(())
}
