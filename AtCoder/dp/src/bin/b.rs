use memoise::memoise;
use proconio::input;
use std::cmp;
use std::isize::MAX;
const INF: isize = MAX / 3;

#[memoise (0 <= n <= 100000)]
// dp[n] := n番目(0-based)の足場に到達するのに必要な最小のコスト
fn dp(n: usize, k: usize, h: &Vec<isize>) -> isize {
        if n == 0 {
                return 0;
        }

        let mut res = INF;
        for i in 1..=cmp::min(k, n) {
                res = cmp::min(res, (h[n - i] - h[n]).abs() + dp(n - i, k, h));
        }
        res
}

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                h: [isize; n],
        }

        // Solve
        let ans = dp(n - 1, k, &h);

        // Output
        println!("{}", ans);
}
