// use chrono::{DateTime, Local};
// use proconio::input;

fn main() {
    let v = vec!["A", "B", "C", "D"];
    for i in 0..1 << v.len() {
        for (j, el) in v.iter().enumerate() {
            if (1 << j) & i != 0 {
                print!("{}", el);
            }
        }
        println!();
    }
}
