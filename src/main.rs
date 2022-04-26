// use chrono::{DateTime, Local};
// use proconio::input;

fn main() {
    let v = vec!["A", "B", "C", "D"];
    // 1 << d := pow(2, d)
    for i in 0..1 << v.len() {
        println!("===");
        // -- (1)
        for (j, el) in v.iter().enumerate() {
            // iのj番目ビットが立っているか -> jがiに含まれるか
            println!("i: {}", i);
            println!("j: {}", j);
            if (1 << j) & i == 0 {
                // -- (2)
                // 又は if i >> j & 1 {
                print!("{}", el);
            }
        }
        println!();
    }
}
