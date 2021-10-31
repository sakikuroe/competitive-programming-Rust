use memoise::memoise;
use proconio::input;
use std::cmp;

#[memoise (0 <= n <= 100000, 0 <= m <= 2)]
// dp[n][m] := n日目(0-based)にmの活動をするときの幸福度の最大値
fn dp(n: usize, m: usize, abc_list: &Vec<Vec<usize>>) -> usize {
        if n == 0 {
                return abc_list[n][m];
        }

        let mut res = 0;
        for i in 0..3 {
                if i == m {
                        continue;
                }
                res = cmp::max(res, dp(n - 1, i, abc_list) + abc_list[n][m]);
        }
        res
}

fn main() {
        // Input
        input! {
                n: usize,
                abc_list: [[usize; 3]; n],
        }

        // Solve
        let ans = {
                let mut res = 0;
                for m in 0..3 {
                        res = cmp::max(res, dp(n - 1, m, &abc_list))
                }
                res
        };

        // Output
        println!("{}", ans);
}
