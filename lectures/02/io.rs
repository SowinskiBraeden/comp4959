use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines() {
    let mut line = String::new();

    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        print!("{line}");
        line.clear();
    }
}

fn lines() -> io::Result<Vec<String>> {
    io::stdin().lines().collect()
}

fn lines2() -> Vec<String> {
    io::stdin().lines().filter_map(|x| x.ok()).collect()
}

fn file_lines(file: &str) -> io::Result<Vec<String>> {
    io::BufReader::new(File::open(file)?).lines().collect()
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    println!("{args:?}");

    if args.len() > 0 {
        println!("{:?}", file_lines(&args[0]));
    } else {
        println!("{:?}", lines());
    }
}
