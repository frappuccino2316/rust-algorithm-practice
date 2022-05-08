use proconio::input;

fn main() {
    input! {
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        x3: i64,
        y3: i64,
        x4: i64,
        y4: i64,
    }

    let c = judge(x1, y1, x2, y2, x3, y3);
    let d = judge(x1, y1, x2, y2, x4, y4);

    if c * d <= 0 {
        println!("交差しています");
    } else {
        println!("交差していません")
    }
}

fn judge(x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> i64 {
    // y-y1=((y1-y2)/(x1-x2))(x-x1) -> (y-y1) * (x1-x2) - (y1-y2) * (x-x1)の正負をチェックする
    (y - y1) * (x1 - x2) - (y1 - y2) * (x - x1)
}
