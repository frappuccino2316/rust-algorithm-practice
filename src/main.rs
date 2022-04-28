use proconio::input;

// Nが３個以上の場合の最大公約数を出す
fn main() {
    input! {
        mut n: i64,
    }

    let mut number_list = Vec::new();
    let mut result = 0;

    for _ in 0..n {
        input! {
            a: i64,
        }
        number_list.push(a);
    }

    for j in 1..n {
        if j == 1 {
            result = gcd(number_list[0], number_list[1]);
        } else {
            result = gcd(result, number_list[j as usize]);
        }
    }

    println!("{}", result);
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a >= 1 && b >= 1 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    if a >= 1 {
        a
    } else {
        b
    }
}
