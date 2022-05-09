use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    }

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut total: i64 = 0;

    for i in 0..n as usize {
        if i == 0 {
            a.push(0);
        } else {
            input! {
                ai: i64,
            }
            a.push(ai);
        }
    }

    for _ in 0..(m) as usize {
        input! {
            bi: usize,
        }
        b.push(bi);
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    let total_sum = total_sum(a);

    for i in 0..b.len() {
        if i != 0 {
            let start = if b[i - 1] < b[i] {
                b[i - 1] - 1
            } else {
                b[i] - 1
            };
            let end = if b[i - 1] < b[i] {
                b[i] - 1
            } else {
                b[i - 1] - 1
            };

            total += total_sum[end] - total_sum[start];
        }
    }

    println!("total move distance is {}km.", total);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}

fn total_sum(a: Vec<i64>) -> Vec<i64> {
    let mut t = Vec::new();
    for i in 0..a.len() {
        if i == 0 {
            t.push(a[i]);
        } else {
            t.push(t[i - 1] + a[i]);
        }
    }
    t
}
