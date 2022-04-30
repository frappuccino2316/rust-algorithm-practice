use proconio::input;

fn main() {
    input! {
        mut n: i64,
    }

    let mut number_list = Vec::new();
    let mut gcd_value = 0;
    let mut lcm_value = 0;

    for _ in 0..n {
        input! {
            a: i64,
        }
        number_list.push(a);
    }

    for j in 1..n {
        if j == 1 {
            gcd_value = gcd(number_list[0], number_list[1]);
            lcm_value = lcm(number_list[0], number_list[1], gcd_value);
        } else {
            gcd_value = gcd(gcd_value, number_list[j as usize]);
            lcm_value = lcm(
                lcm_value,
                number_list[j as usize],
                gcd(lcm_value, number_list[j as usize]),
            );
        }
    }

    println!("gcd: {}, lcm: {}", gcd_value, lcm_value);
}

fn gcd(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64, gcd_value: i64) -> i64 {
    a * b / gcd_value
}
