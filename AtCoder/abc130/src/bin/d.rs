use proconio::input;
use superslice::*;

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                a_list: [usize; n],
        }

        // Initialize
        let s_list = {
                let mut res = vec![0];
                for i in 0..n {
                        res.push(res[i] + a_list[i]);
                }

                res
        };

        // Solve
        let ans = {
                let mut res = 0;
                for i in 0..n {
                        res += (n + 1) - s_list.lower_bound(&(s_list[i] + k));
                }

                res
        };

        // Output
        println!("{}", ans);
}
