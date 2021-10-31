use memoise::memoise;
use proconio::input;
use std::cmp;

#[memoise (0 <= n <= 100, 0 <= m <= 100000)]
// dp[n][m] := n番目(0-based)までの商品で、重さがm以下となるように選ぶときの価値の総和の最大値
fn dp(n: usize, m: usize, wv_list: &Vec<Vec<usize>>) -> usize {
        if n == 0 {
                if wv_list[n][0] <= m {
                        return wv_list[n][1];
                } else {
                        return 0;
                }
        }

        if wv_list[n][0] <= m {
                return cmp::max(
                        dp(n - 1, m - wv_list[n][0], wv_list) + wv_list[n][1],
                        dp(n - 1, m, wv_list),
                );
        } else {
                return dp(n - 1, m, wv_list);
        }
}

fn main() {
        // Input
        input! {
                n: usize, w: usize,
                wv_list: [[usize; 2]; n],
        }

        // Solve
        let ans = dp(n - 1, w, &wv_list);

        // Output
        println!("{}", ans);
}
