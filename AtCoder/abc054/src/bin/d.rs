use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

fn main() {
        // Input
        input! {
                n: usize, ma: usize, mb: usize,
                abc_list: [(usize, usize, usize); n],
        }

        // Solve
        let dp = {
                let mut dp = vec![vec![vec![INF; 401]; 401]; 41];
                dp[0][0][0] = 0;
                for i in 1..=n {
                        let (a, b, c) = abc_list[i - 1];
                        for j in 0..=400 {
                                for k in 0..=400 {
                                        if j < a || k < b {
                                                dp[i][j][k] = dp[i - 1][j][k];
                                        } else {
                                                dp[i][j][k] = cmp::min(
                                                        dp[i][j][k],
                                                        cmp::min(
                                                                dp[i - 1][j][k],
                                                                dp[i - 1][j - a][k - b] + c,
                                                        ),
                                                );
                                        }
                                }
                        }
                }

                dp
        };

        let ans = {
                let mut res = INF;
                for i in 1..=cmp::min(400 / ma, 400 / mb) {
                        res = cmp::min(res, dp[n][i * ma][i * mb]);
                }

                res
        };

        // Output
        if ans != INF {
                println!("{}", ans);
        } else {
                println!("-1");
        }
}
