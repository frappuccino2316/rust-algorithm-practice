use proconio::input;

fn main() {
    input! {
        mut m: i64,
        mut n: i64,
    }
    println!("{}", gcd(m, n));
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a >= 1 && b >= 1 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    if a >= 1 {
        a
    } else {
        b
    }
}
