use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut list = Vec::new();
    let mut index = 1;

    while 2_i64.pow(index) <= 10_i64.pow(18) {
        list.push(2_i64.pow(index) - 1);
        index += 1;
    }

    if list.iter().any(|e| *e == n) {
        println!("Second");
    } else {
        println!("First");
    }
}
