use proconio::input;

fn main() {
    input! {
        n: i64,
        x: i64,
        y: i64,
    }

    let mut i = 1;

    while i < n {
        if i % x == 0 || i % y == 0 {
            println!("{}", i);
        }
        i += 1;
    }
}
