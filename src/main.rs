use proconio::input;

fn main() {
    input! {
        n: i64,
        s: i64,
    }

    let n_usize = n as usize;
    let s_usize = s as usize;

    let mut card_list = Vec::new();

    for i in 0..n + 1 {
        if i == 0 {
            card_list.push(0);
        } else {
            input! {
                a: i64,
            }
            card_list.push(a);
        }
    }

    let mut dp = Vec::new();
    for _ in 0..n + 1 {
        dp.push(vec![false; s_usize + 1]);
    }

    dp[0][0] = true;

    for i in 1..n_usize + 1 {
        for j in 0..s_usize + 1 {
            if (j as i64) < card_list[i] {
                dp[i][j] = dp[i - 1][j];
            } else if (j as i64) >= card_list[i] {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - (card_list[i] as usize)];
            }
        }
    }

    println!("{:?}", dp);
    let result = dp[n_usize][s_usize];
    println!("card can add s?: {}.", result);
}
