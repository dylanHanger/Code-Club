use std::collections::BTreeSet;

fn main() {
    let factors = prime_factors(600_851_475_143);
    println!("{:?}", factors);
}

fn prime_factors(n: u64) -> BTreeSet<u64> {
    let mut n = n;
    let mut factors = BTreeSet::new();
    while n % 2 == 0 {
        factors.insert(2);
        n /= 2;
    }

    for i in (3..).step_by(2) {
        if i > n / i {
            break;
        }
        while n % i == 0 {
            factors.insert(i);
            n /= i;
        }
    }

    if n > 2 {
        factors.insert(n);
    }
    return factors;
}
