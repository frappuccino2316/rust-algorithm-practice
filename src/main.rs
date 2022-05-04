use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: i64,
    }

    let n_usize = n as usize;
    let mut scaffold = vec![0_i64; n_usize + 1];

    for i in scaffold.iter_mut().skip(1).take(n_usize) {
        input! {
            h: i64,
        }
        *i = h;
    }

    let mut result = vec![0; n_usize + 1];

    for i in 1..n_usize + 1 {
        if i == 1 {
            result[i] = 0;
        }
        if i == 2 {
            let difference = scaffold[i] - scaffold[i - 1];
            result[i] = difference.abs();
        }
        if i >= 3 {
            let step_from_one_previous = result[i - 1] + (scaffold[i] - scaffold[i - 1]).abs();
            let step_from_two_previous = result[i - 2] + (scaffold[i] - scaffold[i - 2]).abs();
            result[i] = min(step_from_one_previous, step_from_two_previous);
        }
    }

    println!("min step: {}", result[n_usize]);
}
