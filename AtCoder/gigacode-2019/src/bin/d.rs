use proconio::input;
use std::cmp;

fn main() {
    // Input
    input! {
        h: usize, w: usize, k: usize, v: usize,
        mut a_mat: [[usize; w]; h],
    }

    // Initialize
    let a_mat = {
        for i in 0..h {
            for j in 0..w {
                a_mat[i][j] += k;
            }
        }
        a_mat
    };

    // Solve
    let sum_mat: Vec<Vec<usize>> = {
        let mut res: Vec<Vec<usize>> = vec![vec![0; w + 1]; h + 1];
        for i in 0..h {
            for j in 0..w {
                res[i + 1][j + 1] = a_mat[i][j];
            }
        }

        for i in 0..h {
            for j in 0..w {
                res[i + 1][j + 1] += res[i][j + 1] + res[i + 1][j] - res[i][j];
            }
        }
        res
    };

    let mut ans: usize = 0;
    for i in 0..h {
        for j in 0..w {
            for k in i + 1..=h {
                for l in j + 1..=w {
                    if sum_mat[k][l] + sum_mat[i][j] - sum_mat[k][j] - sum_mat[i][l] <= v {
                        ans = cmp::max(ans, (k - i) * (l - j));
                    }
                }
            }
        }
    }

    // Output
    println!("{}", ans);
}
