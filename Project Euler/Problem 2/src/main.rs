const LIMIT: i32 = 4_000_000;
fn main() {
    println!("{}", smallbrain());
    println!("{}", bigbrain());
}

fn smallbrain() -> i32 {
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
    // Technically less operations, but I suspect the casting and rounding might slow it down
    // Plus, this is susceptible to precision errors and all kinds of juicy things, but it works up to at least 400M
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
