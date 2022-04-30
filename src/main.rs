use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut red_count = 0;
    let mut blue_count = 0;
    let mut yellow_count = 0;

    for _ in 0..n {
        input! {
            a: i64,
        }

        match a {
            1 => red_count += 1,
            2 => blue_count += 1,
            3 => yellow_count += 1,
            _ => println!("no matched"),
        }
    }

    let result = red_count * (red_count - 1) / 2
        + blue_count * (blue_count - 1) / 2
        + yellow_count * (yellow_count - 1) / 2;

    println!(
        "if pick up two card, same color pattern is {} pattern",
        result
    );
}
