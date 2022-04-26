use chrono::{DateTime, Local};
use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let local_datetime_before: DateTime<Local> = Local::now();

    is_prime(n);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}

fn is_prime(x: i64) {
    for i in 2..x + 1 {
        if i == 2 {
            println!("2");
            continue;
        }

        let f = i as f64;
        let m = f.sqrt() as i64;

        for j in 2..i {
            if m < 2 {
                println!("{}", i);
                break;
            }
            if i % j == 0 {
                break;
            }
            if j == m {
                println!("{}", i);
            }
        }
    }
}
