use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut b = Vec::new();
    let mut r = Vec::new();

    for _ in 0..n {
        input! {
            a: i64,
        }
        b.push(a);
    }

    for _ in 0..n {
        input! {
            a: i64,
        }
        r.push(a);
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    let n = n as f64;
    let mut b_total = 0;
    for i in b {
        b_total += i;
    }

    let mut r_total = 0;
    for i in r {
        r_total += i;
    }

    let b_total = b_total as f64;
    let r_total = r_total as f64;

    println!("{}", (b_total + r_total) / n);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
