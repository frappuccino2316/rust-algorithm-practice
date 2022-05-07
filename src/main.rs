use std::vec;

use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let n_usize = n as usize;

    let mut study_point = Vec::new();
    for _ in 0..n {
        input! {
            p: i64,
        }
        study_point.push(p);
    }

    let mut max_point = vec![0; study_point.len()];

    for i in 0..n_usize {
        if i == 0 {
            max_point[i] = study_point[i];
        } else if i == 1 {
            if max_point[i - 1] >= study_point[i] {
                max_point[i] = max_point[i - 1];
            } else {
                max_point[i] = study_point[i];
            }
        } else if max_point[i - 1] >= study_point[i] + max_point[i - 2] {
            max_point[i] = max_point[i - 1];
        } else {
            max_point[i] = study_point[i] + max_point[i - 2];
        }
    }

    println!("max is {}.", max_point[n_usize - 1]);
}
