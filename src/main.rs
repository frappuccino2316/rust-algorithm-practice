use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
    }

    let mut a = Vec::new();

    let mut first = vec![-1; n + 1];
    let mut second = vec![-1; n + 1];

    let mut count = 0_i64;
    let mut current = 1_i64;

    for _ in 0..n {
        input! {
            ai: i64,
        }
        a.push(ai);
    }

    loop {
        if first[current as usize] == -1 {
            first[current as usize] = count;
        } else if second[current as usize] == -1 {
            second[current as usize] = count;
        }

        if count == k {
            println!("{}", current as usize);
            break;
        } else if second[current as usize] != -1
            && (k - first[current as usize]) % (second[current as usize] - first[current as usize])
                == 0
        {
            println!("{}", current);
            break;
        }

        current = a[(current as usize) - 1];
        count += 1;
    }
}
