use proconio::input;
use superslice::{self, Ext};

fn main() {
        // Input
        input! {
            n: usize,
            mut l_list: [usize; n],
        }

        // Initialize
        l_list.sort();

        // Solve
        let ans = {
                let mut res = 0;
                for i in 0..n - 2 {
                        for j in i + 1..n - 1 {
                                let c = l_list.lower_bound(&(l_list[i] + l_list[j])) - 1;
                                if c > j {
                                        res += c - j;
                                }
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
