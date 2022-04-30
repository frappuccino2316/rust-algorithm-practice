use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut one_hundred_count = 0;
    let mut two_hundred_count = 0;
    let mut three_hundred_count = 0;
    let mut four_hundred_count = 0;

    for _ in 0..n {
        input! {
            a: i64,
        }

        match a {
            100 => one_hundred_count += 1,
            200 => two_hundred_count += 1,
            300 => three_hundred_count += 1,
            400 => four_hundred_count += 1,
            _ => println!("no matched"),
        }
    }

    println!(
        "500 pattern is {}",
        one_hundred_count * four_hundred_count + two_hundred_count * three_hundred_count
    );
}
