use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut point_list = Vec::new();

    for _ in 0..n {
        input! {
            x: f64,
            y: f64,
        }
        point_list.push((x, y));
    }

    let mut nearest_distance = 2_f64.powf(60.0);

    for i in 0..(n as usize) - 1 {
        for j in (i as usize) + 1..(n as usize) {
            let distance = ((point_list[j].0 - point_list[i].0).powi(2)
                + (point_list[j].1 - point_list[i].1).powi(2))
            .sqrt();

            if nearest_distance > distance {
                nearest_distance = distance;
            }
        }
    }

    println!("{}", nearest_distance);
}
