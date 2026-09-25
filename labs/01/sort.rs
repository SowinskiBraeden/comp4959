use std::env;
use std::fs;
use std::io;

fn record(line: &str) -> Option<(String, String, u8)> {
    let mut fields = line.split_whitespace();
    let first_name = fields.next()?;
    let last_name = fields.next()?;
    let score: u8 = fields.next()?.parse().ok()?;

    if score > 100 {
        return None;
    }

    Some((first_name.to_string(), last_name.to_string(), score))
}

fn main() {
    let input = match env::args().nth(1) {
        Some(path) => fs::read_to_string(path).unwrap_or_default(),
        None => io::read_to_string(io::stdin()).unwrap_or_default(),
    };

    let mut records: Vec<(String, String, u8)> = input.lines().filter_map(record).collect();
    records.sort();
    records.dedup();
    records.sort_by_key(|r| r.2);

    for (first_name, last_name, score) in records {
        println!("{} {}, {}", score, last_name, first_name);
    }
}
