use std::collections::BTreeMap;

fn main() {
    let start = std::time::Instant::now();
    let primes = sieve(1_000_000);
    println!("Generated primes in {:?}", start.elapsed());
    match find_smallest(2, &primes) {
        Some((family, idx)) => {
            println!(
                "Found {} to be the smallest prime, with {} as the replacement.\n{:?}",
                family[0], idx, family
            );
        }
        None => println!("Found no valid prime"),
    }
}

fn find_smallest(digits: usize, primes: &BTreeMap<u32, bool>) -> Option<(Vec<u32>, usize)> {
    for prime in primes.keys() {
        if prime < &(10_u32.pow(digits as u32)) {
            continue;
        }
        print!("{:06}\r", prime);
        let families = generate_prime_families(*prime, digits, &primes);
        for (idx, family) in families.iter().enumerate() {
            if family.len() == 8 {
                return Some((family.clone(), idx));
            }
        }
    }
    None
}

fn generate_prime_families(n: u32, digits: usize, primes: &BTreeMap<u32, bool>) -> Vec<Vec<u32>> {
    let mut family = vec![vec![]; 10];
    for d in 0..10 {
        family[d].append(&mut filter_primes(
            generate_replacements(n, digits, d as u32),
            primes,
        ));
    }
    return family;
}

fn filter_primes(candidates: Vec<u32>, primes: &BTreeMap<u32, bool>) -> Vec<u32> {
    candidates.into_iter().filter(|c| primes[c]).collect()
}

fn generate_replacements(n: u32, digits: usize, replace_with: u32) -> Vec<u32> {
    let n_str = n.to_string();
    let wc = generate_wildcards(n_str, digits);

    wc.iter()
        .filter(|s| !(s.starts_with("*") && replace_with == 0))
        .map(|s| s.replace("*", &replace_with.to_string()).parse().unwrap())
        .collect()
}

fn generate_wildcards(s: String, n: usize) -> Vec<String> {
    if n == 0 {
        return vec![s];
    } else if s.len() == n {
        // Return a single string where every digit is replaced by a wildcard
        return vec![(0..n).map(|_| "*").collect()];
    } else if s.len() > n {
        // Recursively generate wildcards for the remaining digits
        let mut wildcards = vec![];

        // Leave the first digit, replace the rest
        let s2 = s.clone();
        wildcards.append(
            &mut generate_wildcards(s2[1..].to_string(), n)
                .iter()
                .map(|wc| {
                    // Get the first character from s2
                    let c = s2.chars().next().unwrap();
                    format!("{}{}", c, wc)
                })
                .collect(),
        );

        // Replace the first digit
        let mut s3 = s.clone();
        s3.replace_range(0..1, "*");
        wildcards.append(
            &mut generate_wildcards(s3[1..].to_string(), n - 1)
                .iter()
                .map(|wc| format!("*{}", wc))
                .collect(),
        );

        return wildcards;
    } else {
        panic!("Cannot replace {} characters of \"{}\"", n, s);
    }
}

// The Sieve of Eratosthenes for finding all primes less than n
fn sieve(n: u32) -> BTreeMap<u32, bool> {
    let mut primes = vec![true; n as usize];
    primes[0] = false;
    primes[1] = false;
    for i in 2.. {
        if i > n / i {
            // Only go up to i > sqrt(n), i.e. i*i > n === i > n / i
            break;
        }
        if primes[i as usize] {
            for j in (i * i..).step_by(i as usize) {
                if j >= n {
                    break;
                }
                primes[j as usize] = false;
            }
        }
    }
    return primes
        .iter()
        .enumerate()
        .map(|(i, &p)| (i as u32, p))
        .collect();
}
