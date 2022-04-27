use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    println!("{}", is_prime(n));
}

fn is_prime(x: i64) -> bool {
    let f = x as f64;
    let limit = f.sqrt() as i64;

    for i in 2..limit + 1 {
        if x % i == 0 {
            return false;
        }
    }
    true
}
