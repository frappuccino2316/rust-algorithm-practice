use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: i64,
        max_weight: i64,
    }

    let n_usize = n as usize;
    let max_weight_usize = max_weight as usize;

    let mut weight_list = Vec::new();
    let mut value_list = Vec::new();

    for i in 0..n + 1 {
        if i == 0 {
            weight_list.push(0);
            value_list.push(0);
        } else {
            input! {
                w: i64,
                v: i64,
            }
            weight_list.push(w);
            value_list.push(v);
        }
    }

    let mut dp = Vec::new();
    for _ in 0..n + 1 {
        dp.push(vec![-(2_i64.pow(30)); max_weight_usize + 1]);
    }

    dp[0][0] = 0;

    for i in 1..n_usize + 1 {
        for j in 0..max_weight_usize + 1 {
            if (j as i64) < weight_list[i] {
                dp[i][j] = dp[i - 1][j];
            } else if (j as i64) >= weight_list[i] {
                dp[i][j] = max(
                    dp[i - 1][j],
                    dp[i - 1][j - (weight_list[i] as usize)] + value_list[i],
                );
            }
        }
    }

    let mut answer = 0_i64;
    println!("{:?}", dp);
    for k in 0..max_weight_usize + 1 {
        answer = max(answer, dp[n_usize][k]);
    }
    println!("Answer is {}.", answer);
}
