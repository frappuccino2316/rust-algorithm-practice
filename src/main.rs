use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        // a: 時針の長さ、b: 分針の長さ、h: 時刻の時、m: 時刻の分
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let h_radian = (2.0 * PI / 12.0) * h + ((2.0 * PI / 12.0 / 60.0) * m);
    let m_radian = (2.0 * PI / 60.0) * m;
    let target_radian = if (h_radian - m_radian).abs() < PI {
        (h_radian - m_radian).abs()
    } else {
        2.0 * PI - (h_radian - m_radian).abs()
    };

    println!("h_radian: {}", h_radian / PI);
    println!("m_radian: {}", m_radian / PI);
    println!("target_radian: {}", target_radian / PI);

    let result = (a.powi(2) + b.powi(2) - (2.0 * a * b * target_radian.cos())).sqrt();

    println!("result: {}cm.", result);
}
