use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        n: f64,
        a: f64,
        b: f64
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    println!("{}", ((a / 3.0) + ((b * 2.0) / 3.0)) * n);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
