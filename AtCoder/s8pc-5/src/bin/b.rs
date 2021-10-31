use proconio::input;
use std::f64::MAX;
const INF: f64 = MAX;

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        xyr_list: [(f64, f64, f64); n],
        xy_list: [(f64, f64); m],
    }

    // Solve
    let r_list: Vec<f64> = {
        let mut r_list: Vec<f64> = vec![];

        // 半径が決まっている円
        for &(_, _, r) in &xyr_list {
            r_list.push(r);
        }

        // 半径が決まっていない円
        for (i, &(x1, y1)) in xy_list.iter().enumerate() {
            let mut r = INF;
            for (j, &(x2, y2)) in xy_list.iter().enumerate() {
                if i == j {
                    continue;
                }
                let dist = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt() / 2.0;
                if r > dist {
                    r = dist;
                }
            }
            for &(x2, y2, r2) in &xyr_list {
                let dist = ((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1)).sqrt() - r2;
                if r > dist {
                    r = dist;
                }
            }
            r_list.push(r);
        }

        r_list
    };

    let mut ans = INF;
    for r in r_list {
        ans = ans.min(r);
    }

    // Output
    println!("{}", ans);
}
