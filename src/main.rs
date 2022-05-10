use chrono::{DateTime, Local};
// use proconio::input;

fn main() {
    let local_datetime_before: DateTime<Local> = Local::now();

    let r = 1000.0_f64;
    // 10^0.3 = 10√1000になる
    let mut a = 1000_f64;

    for i in 0..75 {
        // 点 (a, f(a))の座標
        let x = a;
        let y = a * a * a * a * a * a * a * a * a * a;

        // 上記座標の接戦を出す y = session_a * x + session_b
        let session_a = 9.0 * x * x * x * x * x * x * x * x * x;
        let session_b = y - session_a * x;

        let next_a = (r - session_b) / session_a;

        println!("Step{}: a={} -> {}.", i, a, next_a);

        a = next_a;
    }

    let answer = 10.0_f64.powf(0.3);
    println!("answer: {}", answer);

    let local_datetime_after: DateTime<Local> = Local::now();

    let code_duration = local_datetime_after - local_datetime_before;
    println!("time: {}", code_duration.num_milliseconds());
}
