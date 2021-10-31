use memoise::memoise;
use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

#[memoise (0 <= n <= 99, 0 <= m <= 100000)]
// dp[n][m] := n番目(0-based)までの商品で、価値がmとなるように選ぶときの重さの総和の最小値
fn dp(n: usize, m: usize, wv_list: &Vec<Vec<usize>>) -> usize {
        if m == 0 {
                return 0;
        }
        if n == 0 {
                if wv_list[n][1] == m {
                        return wv_list[n][0];
                } else {
                        return INF;
                }
        }

        if wv_list[n][1] <= m {
                return cmp::min(
                        dp(n - 1, m - wv_list[n][1], wv_list) + wv_list[n][0],
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
        let ans: usize = {
                let mut res = 0;
                for i in 0..=100000 {
                        if dp(n - 1, i, &wv_list) <= w {
                                res = cmp::max(res, i);
                        }
                }
                res
        };

        // Output
        println!("{}", ans);
}
