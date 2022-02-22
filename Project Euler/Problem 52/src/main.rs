use std::time::Instant;

fn main() {
    let start = Instant::now();
    let x = find_smallest(0);
    println!(
        "Smallest is: ({}, {}, {}, {}, {}, {}) (took {:?})",
        x,
        2 * x,
        3 * x,
        4 * x,
        5 * x,
        6 * x,
        start.elapsed()
    );
}

fn find_smallest(lower_bound: i32) -> i32 {
    let mut i = lower_bound;
    'outer: loop {
        i += 1;
        for m in (2..=6).rev() {
            if !is_permutation(i, m * i) {
                continue 'outer;
            }
        }
        return i;
    }
}

fn is_permutation(i: i32, j: i32) -> bool {
    let i_digits = get_digits(i);
    let j_digits = get_digits(j);

    if i_digits.len() != j_digits.len() {
        false
    } else {
        for (a, b) in i_digits.iter().zip(j_digits.iter()) {
            if a != b {
                return false;
            }
        }
        return true;
    }
}

fn get_digits(i: i32) -> Vec<u32> {
    let mut digits: Vec<u32> = i
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    digits.sort();
    digits
}
