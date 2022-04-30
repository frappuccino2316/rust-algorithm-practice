use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut v = Vec::new();

    for _ in 0..n {
        input! {
            a: i64,
        }
        v.push(a);
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    let mut i = 0;
    let mut count = 0;

    while i < v.len() {
        let mut j = i + 1;

        while j < v.len() {
            if v[i] + v[j] == 100000 {
                count += 1;
            }
            j += 1;
        }

        i += 1;
    }

    println!("{}", count);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
