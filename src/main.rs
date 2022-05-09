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

    for _ in 0..(n - 1) as usize {
        input! {
            ai: i64,
        }
        a.push(ai);
    }

    for _ in 0..(m) as usize {
        input! {
            bi: usize,
        }
        b.push(bi);
    }

    let local_datetime_before: DateTime<Local> = Local::now();

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

            total += sum(a[start..end].to_vec());
        }
    }

    println!("total move distance is {}km.", total);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}

fn sum(a: Vec<i64>) -> i64 {
    if a.len() == 1 {
        a[0]
    } else {
        let r = sum(a[0..a.len() - 1].to_vec());

        a[a.len() - 1] + r
    }
}
