use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX;

fn main() {
        // Input
        input! {
            n: usize, m: usize,
            abc_list: [(usize, usize, usize); m],
        }

        // Initialize
        let mut d_mat = vec![vec![INF / 4; n]; n];
        let abc_list =
                abc_list.into_iter()
                        .map(|(a, b, c)| (a - 1, b - 1, c))
                        .collect::<Vec<(usize, usize, usize)>>();
        for i in 0..n {
                d_mat[i][i] = 0;
        }

        // Solve
        let mut ans = 0;
        for (a, b, c) in abc_list {
                d_mat[a][b] = c;
        }

        // Floyd-Warshall Algorithm
        for k in 0..n {
                for i in 0..n {
                        for j in 0..n {
                                d_mat[i][j] = cmp::min(d_mat[i][j], d_mat[i][k] + d_mat[k][j]);
                        }
                }
                for i in 0..n {
                        for j in 0..n {
                                if d_mat[i][j] != INF / 4 {
                                        ans += d_mat[i][j];
                                }
                        }
                }
        }

        // Output
        println!("{}", ans);
}
