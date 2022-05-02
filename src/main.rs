use proconio::input;
use rand::Rng;

fn main() {
    // 試行回数
    input! {
        n: i64,
    }

    let n_f64 = n as f64;
    let mut m = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..n {
        let x = rng.gen_range(0.0..=1.0);
        let y = rng.gen_range(0.0..=1.0);

        if x * x + y * y <= 1.0 {
            m += 1.0;
        }
    }
    println!("π: {}", 4.0 * m / n_f64);
}
