use proconio::input;
use std::f64::consts::PI;

fn main() {
    // Input
    input! {
        a: f64, b: f64, x: f64,
    }

    // Solve
    // xが直方体の半分以上の体積があるかどうか判断して、角度を計算
    let ans = if x >= a * a * b / 2.0 {
        (2.0 * b / a - 2.0 * x / (a * a * a)).atan()
    } else {
        (a * b * b / (2.0 * x)).atan()
    };

    // Output
    // 度数法に直して出力
    println!("{}", ans * 180.0 / PI);
}
