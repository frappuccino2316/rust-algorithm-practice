use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64
    }

    let mut i = 1;
    let mut j = 1;
    let mut c = 0;

    while i <= n {
        while j <= n + 1 {
            if i + j <= a {
                c += 1;
            }
            j += 1;
        }
        i += 1;
    }

    println!("{}", c);
}
