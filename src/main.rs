use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let n_usize = n as usize;

    let mut v = vec![0; n_usize];

    for i in 0..n_usize {
        if i == 0 {
            v[i] = 1;
        }
        if i >= 1 {
            v[i] = 2;
        }
        if i >= 2 {
            v[i] = v[i - 1] + v[i - 2];
        }
    }

    println!("{}", v[n_usize - 1]);
}
