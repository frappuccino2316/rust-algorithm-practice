use proconio::input;

fn main() {
    input! {
        x1: f64,
        y1: f64,
        r1: f64,
        x2: f64,
        y2: f64,
        r2: f64,
    }

    let center_distance = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

    if center_distance + r1 < r2 || center_distance + r2 < r1 {
        println!("1");
    } else if center_distance + r1 == r2 || center_distance + r2 == r1 {
        println!("2");
    } else if center_distance < r1 + r2 && (center_distance + r1 > r2 || center_distance + r2 > r1)
    {
        println!("3");
    } else if center_distance == r1 + r2 {
        println!("4");
    } else if center_distance > r1 + r2 {
        println!("5");
    }
}
