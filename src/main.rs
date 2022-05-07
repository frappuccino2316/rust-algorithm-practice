use proconio::input;

fn main() {
    input! {
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        cx: f64,
        cy: f64,
    }

    let answer: f64;

    if bx * ax + by * ay < 0.0 {
        answer = ((bx - ax).powf(2.0) + (by - ay).powf(2.0)).sqrt();
    } else if cx * ax + cy * ay < 0.0 {
        answer = ((cx - ax).powf(2.0) + (by - ay).powf(2.0)).sqrt();
    } else {
        let ab = (bx - ax, by - ay);
        let ac = (cx - ax, cy - ay);
        answer =
            (ab.0 * ac.1 - ab.1 * ac.0).abs() / ((cx - bx).powf(2.0) + (cy - by).powf(2.0)).sqrt();
    }

    println!("answer is {}", answer);
}
