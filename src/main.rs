use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n + 1];
    let mut result = 0;

    for _ in 1..=m {
        input! {
            ai: usize,
            bi: usize,
        }
        g[ai].push(bi);
        g[bi].push(ai);
    }

    for (i, _) in g.iter().enumerate().take(n + 1).skip(1) {
        let mut tmp_count = 0;
        for j in &g[i] {
            if *j < i {
                tmp_count += 1;
            }
        }
        if tmp_count == 1 {
            result += 1;
        }
    }

    println!("{}", result);
}
