use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    let mut r = 1;
    for i in 2..n + 1 {
        r *= i;
    }

    println!("{}", r);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
