use memoise::memoise;
use proconio::input;
use std::cmp;

#[memoise (0 <= n <= 100000)]
// dp[n] := n番目(0-based)の足場に到達するのに必要な最小のコスト
fn dp(n: usize, h: &Vec<isize>) -> isize {
        if n == 0 {
                return 0;
        }
        if n == 1 {
                return (h[n - 1] - h[n]).abs() + dp(n - 1, h);
        }

        return cmp::min(
                (h[n - 2] - h[n]).abs() + dp(n - 2, h),
                (h[n - 1] - h[n]).abs() + dp(n - 1, h),
        );
}

fn main() {
        // Input
        input! {
                n: usize,
                h: [isize; n],
        }

        // Solve
        let ans = dp(n - 1, &h);

        // Output
        println!("{}", ans);
}
