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
    for a in &lines[..lines.len() - 2] {
        for b in &lines[..lines.len() - 1] {
            if a + b == 2020 {
                println!("{}", a * b);
                std::process::exit(0);
            } else if a + b > 2020 {
                break;
            }
        }
    }
    println!("No result found");
}
