use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    let mut count = 0;

    if n < 3 {
        count += 1;
        println!("2");
    } else {
        for i in 3..n + 1 {
            for j in 2..i {
                if i % j == 0 {
                    break;
                }
                if j == i - 1 {
                    count += 1;
                    println!("{}", i);
                }
            }
        }
    }
    println!("count: {}", count);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
