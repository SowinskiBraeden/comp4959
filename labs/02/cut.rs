use std::env;
use std::io;
use std::process;

enum Range {
    Nth(usize),
    From(usize),
    Between(usize, usize),
    To(usize),
}

impl Range {
    fn parse(s: &str) -> Option<Range> {
        match s.split_once('-') {
            None => Some(Range::Nth(number(s)?)),
            Some(("", m)) => Some(Range::To(number(m)?)),
            Some((n, "")) => Some(Range::From(number(n)?)),
            Some((n, m)) => {
                let (n, m) = (number(n)?, number(m)?);
                if n > m {
                    return None;
                }
                Some(Range::Between(n, m))
            }
        }
    }

    fn contains(&self, n: usize) -> bool {
        match *self {
            Range::Nth(x) => n == x,
            Range::From(x) => n >= x,
            Range::Between(x, y) => n >= x && n <= y,
            Range::To(y) => n <= y,
        }
    }
}

// str::parse accepts a leading '+', so check digits ourselves
fn number(s: &str) -> Option<usize> {
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let n: usize = s.parse().ok()?;
    if n == 0 {
        return None;
    }

    Some(n)
}

fn parse_args(arg: &str) -> Option<(bool, Vec<Range>)> {
    let fields = match arg.get(..2)? {
        "-c" => false,
        "-f" => true,
        _ => return None,
    };

    let ranges = arg[2..]
        .split(',')
        .map(Range::parse)
        .collect::<Option<Vec<_>>>()?;

    Some((fields, ranges))
}

fn selected(ranges: &[Range], n: usize) -> bool {
    ranges.iter().any(|r| r.contains(n))
}

fn cut(line: &str, fields: bool, ranges: &[Range]) -> String {
    if fields {
        line.split('\t')
            .enumerate()
            .filter(|(i, _)| selected(ranges, i + 1))
            .map(|(_, f)| f)
            .collect::<Vec<_>>()
            .join("\t")
    } else {
        line.chars()
            .enumerate()
            .filter(|(i, _)| selected(ranges, i + 1))
            .map(|(_, c)| c)
            .collect()
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let (fields, ranges) = match args.as_slice() {
        [arg] => parse_args(arg),
        _ => None,
    }
    .unwrap_or_else(|| {
        eprintln!("usage: cut -c<ranges> | -f<ranges>");
        process::exit(1);
    });

    let input = io::read_to_string(io::stdin()).unwrap_or_default();

    for line in input.split_inclusive('\n') {
        let body = line.trim_end_matches(['\n', '\r']);
        print!("{}{}", cut(body, fields, &ranges), &line[body.len()..]);
    }
}
