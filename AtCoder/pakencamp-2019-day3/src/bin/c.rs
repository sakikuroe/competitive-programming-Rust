use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
            n: usize, m: usize,
            a_mat: [[usize; m]; n],
        }

        // Solve
        let mut ans = 0;
        for i in 0..m - 1 {
                for j in i + 1..m {
                        let mut sum = 0;
                        for k in 0..n {
                                sum += cmp::max(a_mat[k][i], a_mat[k][j]);
                        }
                        ans = cmp::max(ans, sum);
                }
        }

        // Output
        println!("{}", ans);
}
