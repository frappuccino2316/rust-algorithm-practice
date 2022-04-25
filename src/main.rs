use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    let local_datetime_before: DateTime<Local> = Local::now();

    input! {
        n: i64,
        a: i64
    }

    let mut i = 1;
    let mut j = 1;
    let mut c = 0;

    while i <= n {
        while j <= n + 1 {
            if i + j <= a {
                c += 1;
            }
            j += 1;
        }
        i += 1;
    }

    println!("{}", c);
    let local_datetime_after: DateTime<Local> = Local::now();
    let code_duration = local_datetime_after - local_datetime_before;

    println!("time: {}", code_duration.num_milliseconds());
}
