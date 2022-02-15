use std::collections::{BTreeMap, HashSet};

fn main() {
    let start = std::time::Instant::now();
    let primes = sieve(1_000_000);
    println!("Generated primes in {:?}", start.elapsed());

    match find_smallest(8, &primes) {
        Some(family) => {
            println!(
                "Found {} to be the smallest prime, in {:?}.\n{:?}",
                family[0],
                start.elapsed(),
                family
            );
        }
        None => println!("Found no valid prime"),
    }
}

fn find_smallest(family_size: usize, primes: &BTreeMap<u32, bool>) -> Option<Vec<u32>> {
    let mut checked = HashSet::<u32>::new();
    for prime in primes.iter().filter_map(|(k, v)| match v {
        true => Some(k),
        false => None,
    }) {
        if checked.contains(prime) {
            continue;
        }
        print!("{:6}\r", prime);
        for family in generate_families(*prime, primes) {
            for sibling in family.clone() {
                checked.insert(sibling);
            }
            if family.len() == family_size {
                return Some(family);
            }
        }
    }
    None
}

fn generate_families(n: u32, primes: &BTreeMap<u32, bool>) -> Vec<Vec<u32>> {
    let mut families = Vec::new();
    for wildcard in generate_wildcards(n.to_string()) {
        let mut family = Vec::new();
        for d in 0..10 {
            if d == 0 && wildcard.starts_with("*") {
                continue;
            }
            let wc = wildcard.clone().replace("*", &d.to_string());
            family.push(wc.parse().unwrap());
        }
        families.push(filter_primes(family, primes));
    }
    families
}

fn filter_primes(candidates: Vec<u32>, primes: &BTreeMap<u32, bool>) -> Vec<u32> {
    candidates.into_iter().filter(|c| primes[c]).collect()
}

fn generate_wildcards(s: String) -> Vec<String> {
    let mut wildcards = HashSet::new();
    for c in s.chars() {
        let new_s = s.clone().replace(c, "*");
        wildcards.insert(new_s);
    }
    wildcards.into_iter().collect()
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
