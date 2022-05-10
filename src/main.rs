use proconio::input;

fn main() {
    input! {
        t: i64,
        n: i64,
    }

    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut a = Vec::new();
    let mut b = Vec::new();

    for i in 0..n + 1 {
        if i == 0 {
            l.push(0_i64);
            r.push(0_i64);
        } else {
            input! {
                li: i64,
                ri: i64,
            }
            l.push(li);
            r.push(ri);
        }
    }

    for _ in 0..t + 1 {
        a.push(0_i64);
        b.push(0_i64);
    }

    for i in 1..(n as usize) + 1 {
        b[l[i] as usize] += 1;
        b[(r[i] as usize)] -= 1;
    }

    a[0] = b[0];
    for i in 1..(t as usize) + 1 {
        a[i] = a[i - 1] + b[i];
    }

    for (i, _) in a.iter().enumerate().take(t as usize) {
        println!("{}", a[i]);
    }
}
