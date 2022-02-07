const LIMIT: i32 = 1000;
fn main() {
    let mut total = 0;
    for i in 1..LIMIT {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
    }
    println!("{}", total);
}
