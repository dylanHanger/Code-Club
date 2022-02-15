const LIMIT: i32 = 4_000_000;
fn main() {
    // Compare the times of big brain and small brain
    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        mediumbrain();
    }
    let smallbrain_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        mediumbrain();
    }
    let mediumbrain_time = start.elapsed();

    let start = std::time::Instant::now();
    for _ in 0..100_000 {
        bigbrain();
    }
    let bigbrain_time = start.elapsed();

    println!("Brain size\tTime\t\tAnswer");
    println!("Small\t\t{:?}\t{}", smallbrain_time, smallbrain());
    println!("Medium\t\t{:?}\t{}", mediumbrain_time, mediumbrain());
    println!("Big\t\t{:?}\t{}", bigbrain_time, bigbrain());
}

fn smallbrain() -> i32 {
    let mut total = 0;
    let mut fib = [1, 1, 2];
    while fib[2] < LIMIT {
        if fib[2] % 2 == 0 {
            total += fib[2];
        }
        fib[0] = fib[1];
        fib[1] = fib[2];
        fib[2] = fib[0] + fib[1];
    }
    total
}

fn mediumbrain() -> i32 {
    let mut total = 0;
    let mut fib = [1, 1, 2];
    while fib[2] < LIMIT {
        // Every third fibonacci number is even
        // so we just go 3 along the sequence each time.
        // This way, we don't need to check if it is odd
        total += fib[2];
        for _ in 0..3 {
            fib[0] = fib[1];
            fib[1] = fib[2];
            fib[2] = fib[0] + fib[1];
        }
    }
    total
}

fn bigbrain() -> i32 {
    // This is susceptible to precision errors and all kinds of juicy things, but it works
    let phi = (1. + f64::sqrt(5.)) / 2.;
    let phi3 = phi * phi * phi;

    let mut total = 0;
    let mut fib = 2;
    while fib < LIMIT {
        total += fib;
        fib = ((fib as f64) * phi3).round() as i32;
    }
    total
}
