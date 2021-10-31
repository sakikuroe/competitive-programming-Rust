use memoise::memoise;
use proconio::input;
use std::usize::MAX;

use std::cmp;

const INF: usize = MAX;

#[memoise (0 <= i <= 1000, 0 <= t <= 1000)]
// dp(i, t) := i日目(0 <= d <= m)に都市t(0 <= t <= n)に滞在するために必要な疲労度合計の最小値
// 到達できない場合はINFを返す
fn dp(i: usize, t: usize, d_list: &Vec<usize>, c_list: &Vec<usize>) -> usize {
    // 初期条件
    if i == 0 {
        if t == 0 {
            return 0;
        } else {
            return INF;
        }
    }
    if t == 0 {
        return 0;
    }
    if i < t {
        return INF;
    }

    // 前日から滞在し続けるか、前の都市から移動してきたのかで小さい方を返す
    cmp::min(
        dp(i - 1, t, d_list, c_list),
        dp(i - 1, t - 1, d_list, c_list) + d_list[t - 1] * c_list[i - 1],
    )
}

fn solve(d_list: &Vec<usize>, c_list: &Vec<usize>) -> usize {
    let m = c_list.len();
    let n = d_list.len();
    dp(m, n, &d_list, &c_list)
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        d_list: [usize; n],
        c_list: [usize; m],
    }

    // Solve
    let ans = solve(&d_list, &c_list);

    // Output
    println!("{}", ans);
}
