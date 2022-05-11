use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut prime = Vec::new();
    for i in 0..n + 1 {
        if i < 2 {
            prime.push(false)
        } else {
            prime.push(true);
        }
    }

    let mut j = 2_usize;
    while j * j < n.try_into().unwrap() {
        if prime[j] {
            let mut x = j * 2;
            while x <= n.try_into().unwrap() {
                prime[x] = false;
                x += j;
            }
        }
        j += 1;
    }

    for (k, p) in prime.iter().enumerate() {
        if *p {
            println!("{}", k);
        }
    }
}
