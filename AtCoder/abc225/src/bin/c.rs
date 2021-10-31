use proconio::input;
use proconio::marker::Isize1;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                b: [[Isize1;m]; n],
        }

        // Solve
        let mut f = true;
        for i in 0..n {
                for j in 0..m - 1 {
                        if b[i][j + 1] - b[i][j] != 1 {
                                f = false;
                        }
                }
        }

        for i in 0..m {
                for j in 0..n - 1 {
                        if b[j + 1][i] - b[j][i] != 7 {
                                f = false;
                        }
                }
        }

        if b[0][0] % 7 > b[0][m - 1] % 7 {
                f = false;
        }

        // Output
        if f {
                println!("Yes");
        } else {
                println!("No");
        }
}
