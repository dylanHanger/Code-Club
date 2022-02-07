const LIMIT: i32 = 1000;
fn main() {
    println!("{}", smallbrain());
    println!("{}", bigbrain());
}

fn smallbrain() -> i32 {
    let mut total = 0;
    for i in 1..LIMIT {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }
    total
}

fn bigbrain() -> i32 {
    // There are 3 arithmetic series here
    // 3..6..9.... (a)
    // 5..10..15.... (b)
    // 15..30..45.... (c)
    // The terms in (c) get counted twice, since they are divisible by 3 and 5
    // So the final total is S(a) + S(b) - S(c)

    // The last multiple of 3 before 1000 is 999
    let n_a = (LIMIT - 1) / 3; // 333
    let s_a = multiples_sum(3, n_a);

    // The last multiple of 5 before 1000 is 995
    let n_b = (LIMIT - 1) / 5; // 199
    let s_b = multiples_sum(5, n_b);

    // The last multple of 15 before 1000 is 990
    let n_c = (LIMIT - 1) / 15; // 66
    let s_c = multiples_sum(15, n_c);

    s_a + s_b - s_c
}

fn multiples_sum(t0: i32, n: i32) -> i32 {
    // S(t) = n * (t0 + tn) / 2
    // tn = t0 + (n - 1)*d
    let tn = t0 + (n - 1) * t0;
    n * (t0 + tn) / 2
}
