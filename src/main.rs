use proconio::input;

fn main() {
    input! {
        n: i64,
        r: i64,
    }

    let mut result = 1;

    for i in 0..r {
        result *= n - i;
    }

    for i in 0..r {
        result /= r - i;
    }

    println!("nCr: {}", result);
}
