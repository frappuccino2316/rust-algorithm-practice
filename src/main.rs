use proconio::input;

fn main() {
    input! {
        mut n: i64
    }
    let mut result = 0;
    while n != 0 {
        input! {
            a: i64
        }
        result += a;
        n -= 1;
    }
    println!("{}", result);
}
