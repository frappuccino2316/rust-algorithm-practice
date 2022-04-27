use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let f = n as f64;
    let m = f.sqrt() as i64;

    let mut v = vec![];

    for i in 2..n + 1 {
        if i > m {
            break;
        }
        if is_prime(i) {
            v.push(i);
        }
    }

    let mut index = 0;
    let mut result = vec![];
    while index < v.len() {
        if n % v[index] == 0 {
            result.push(v[index]);
            n /= v[index];
        } else {
            index += 1;
        }
    }
    if is_prime(n) && n != 1 {
        result.push(n);
    }

    let list: Vec<String> = result.iter().map(|a| a.to_string()).collect();

    println!("{}", list.join(" × "));
}

fn is_prime(x: i64) -> bool {
    let f = x as f64;
    let limit = f.sqrt() as i64;

    for i in 2..limit + 1 {
        if x % i == 0 {
            return false;
        }
    }
    true
}
