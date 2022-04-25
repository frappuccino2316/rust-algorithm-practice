use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let mut t = 0;

    while n != 0 {
        input! {
            i: i64
        }
        t += i;
        n -= 1;
    }

    println!("{}", t % 100);
}
