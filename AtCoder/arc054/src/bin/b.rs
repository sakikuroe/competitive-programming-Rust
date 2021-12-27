use proconio::input;

fn y(x: f64, p: f64) -> f64 {
    x + p * 2.0f64.powf(-1.0 * x / 1.5)
}

fn y_1(x: f64, p: f64) -> f64 {
    1.0 - p / 1.5 * 2.0f64.powf(-1.0 * x / 1.5) * 2.0f64.ln()
}

fn main() {
    // Input
    input! {
        p: f64,
    }

    // Solve
    let mut ng: f64 = -1.0;
    let mut ok: f64 = p + 1.0;
    while (ok - ng).abs() > 0.00001 {
        let mid = (ok + ng) / 2.0;
        if y_1(mid, p) >= 0.0 {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = if ok >= 0.0 { y(ok, p) } else { p };

    // Output
    println!("{}", ans);
}
