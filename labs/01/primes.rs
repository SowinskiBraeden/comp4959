use std::collections::HashMap;

fn sieve(p: Vec<usize>, mut acc: Vec<usize>, n: usize) -> Vec<usize> {
    if p.len() == 0 {
        return acc;
    }

    let h = p[0];

    if h * h > n {
        acc.extend(p); // acc ++ [h | t] in Elixir impl
        return acc;
    }

    acc.push(h);
    let remaining = p
        .into_iter()
        .skip(1) //
        .filter(|x| x % h != 0)
        .collect();

    sieve(remaining, acc, n)
}

fn primes(m: usize, n: usize) -> Vec<usize> {
    let c = (2..=n).collect();
    sieve(c, Vec::new(), n)
        .into_iter()
        .filter(|x| x > (&m))
        .collect()
}

fn sort_prime(n: usize) -> String {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();
    digits.into_iter().collect()
}

fn main() {
    let p = primes(100_000, 1_000_000);
    println!("{}", p.len());

    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for x in p {
        groups.entry(sort_prime(x)).or_default().push(x);
    }

    let largest = groups.values().max_by_key(|g| g.len()).unwrap();
    println!("{}", largest.len());
}
