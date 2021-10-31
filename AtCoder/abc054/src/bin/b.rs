use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                a_mat: [Chars; n],
                b_mat: [Chars; m],
        }

        // Solve
        let mut f = false;
        for i in 0..=n - m {
                for j in 0..=n - m {
                        let mut matched = 0;

                        for k in 0..m {
                                for l in 0..m {
                                        if a_mat[i + k][j + l] == b_mat[k][l] {
                                                matched += 1;
                                        }
                                }
                        }

                        if matched == m * m {
                                f = true;
                        }
                }
        }

        // Output
        if f {
                println!("Yes");
        } else {
                println!("No");
        }
}
