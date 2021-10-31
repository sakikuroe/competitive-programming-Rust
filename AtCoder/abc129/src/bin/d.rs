use proconio::input;
use proconio::marker::Chars;
use std::cmp;

fn main() {
        // Input
        input! {
                h: usize, w: usize,
                s_list: [Chars; h],
        }

        // Solve
        let light_mat = {
                let mut res = vec![vec![(0, 0); w]; h];
                for i in 0..h {
                        let mut sum = 0;
                        let mut left = 0;
                        for j in 0..w {
                                if s_list[i][j] == '.' {
                                        sum += 1;
                                        if j == w - 1 || s_list[i][j + 1] == '#' {
                                                for k in left..=j {
                                                        res[i][k].0 = sum;
                                                }
                                                sum = 0;
                                                left = j + 2;
                                        }
                                } else {
                                        left = j + 1;
                                }
                        }
                }

                for i in 0..w {
                        let mut sum = 0;
                        let mut left = 0;
                        for j in 0..h {
                                if s_list[j][i] == '.' {
                                        sum += 1;
                                        if j == h - 1 || s_list[j + 1][i] == '#' {
                                                for k in left..=j {
                                                        res[k][i].1 = sum;
                                                }
                                                sum = 0;
                                                left = j + 2;
                                        }
                                } else {
                                        left = j + 1;
                                }
                        }
                }

                res
        };

        let ans = {
                let mut res = 0;
                for i in 0..h {
                        for j in 0..w {
                                res = cmp::max(res, light_mat[i][j].0 + light_mat[i][j].1 - 1);
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
