use std::collections::HashMap;

/*
 * Sieve of Eratosthenes: but the right way
 */

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2;

    while p <= n / p {
        if is_prime[p] {
            let mut multiple = p * p;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += p;
            }
        }

        p += 1;
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(num, &candidate)| candidate.then_some(num))
        .collect()
}

fn primes(m: usize, n: usize) -> Vec<usize> {
    if m > n || n < 2 {
        return Vec::new();
    }
    sieve(n).into_iter().filter(|&p| p >= m).collect()
}

fn sort_digits(n: usize) -> String {
    let mut digits: Vec<char> = n.to_string().chars().collect();
    digits.sort();
    digits.into_iter().collect()
}

fn main() {
    let p = primes(100_000, 1_000_000);
    println!("{}", p.len());

    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
    for x in p {
        groups.entry(sort_digits(x)).or_default().push(x);
    }

    let largest = groups.values().max_by_key(|g| g.len()).unwrap();
    println!("{}", largest.len());
}
