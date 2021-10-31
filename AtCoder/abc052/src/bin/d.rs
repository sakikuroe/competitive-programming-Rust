use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                n: usize, a: usize, b: usize,
                x_list: [usize; n],
        }

        // Solve
        let ans = {
                let mut res = 0;
                for i in 0..n - 1 {
                        res += cmp::min(a * (x_list[i + 1] - x_list[i]), b);
                }

                res
        };

        // Output
        println!("{}", ans);
}
