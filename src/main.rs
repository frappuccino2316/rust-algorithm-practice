use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![0; n + 1];
    let mut answer = 0;

    for i in 1..n + 1 {
        let mut j = i;
        while j <= n {
            v[j] += 1;
            j += i;
        }
    }

    for (k, value) in v.iter().enumerate() {
        if k != 0 {
            answer += k * *value;
        }
    }

    println!("{}", answer);
}
