use memoise::memoise;
use proconio::input;
use std::{cmp, usize::MAX};
const INF: usize = MAX;

#[memoise (0 <= i <= 1000, 0 <= j <= 1000)]
fn dp(i: usize, j: usize, c: usize, mat: &Vec<Vec<usize>>) -> usize {
        if i == 0 || j == 0 {
                return INF / 4;
        }

        cmp::min(mat[i - 1][j - 1], cmp::min(dp(i - 1, j, c, mat) + c, dp(i, j - 1, c, mat) + c))
}

fn main() {
        // Input
        input! {
                h: usize, w: usize, c: usize,
                a_mat: [[usize; w]; h],
        }

        // Initialize
        // b_mat := a_matを上下反転させたもの
        let b_mat = {
                let mut res = vec![vec![0; w]; h];
                for i in 0..h {
                        for j in 0..w {
                                res[i][j] = a_mat[h - 1 - i][j];
                        }
                }
                res
        };

        // Solve
        let mut ans = INF;
        for i in 1..=h {
                for j in 1..=w {
                        ans = cmp::min(
                                ans,
                                cmp::min(
                                        dp(i - 1, j, c, &a_mat) + c + a_mat[i - 1][j - 1],
                                        dp(i, j - 1, c, &a_mat) + c + a_mat[i - 1][j - 1],
                                ),
                        );
                }
        }
        dp_reset();
        for i in 1..=h {
                for j in 1..=w {
                        ans = cmp::min(
                                ans,
                                cmp::min(
                                        dp(i - 1, j, c, &b_mat) + c + b_mat[i - 1][j - 1],
                                        dp(i, j - 1, c, &b_mat) + c + b_mat[i - 1][j - 1],
                                ),
                        );
                }
        }

        // Output
        println!("{}", ans);
}
