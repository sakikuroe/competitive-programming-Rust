use proconio::input;
use std::usize::MAX;
const INF: usize = MAX / 3;

fn main() {
        // Input
        input! {
            n: usize, k: usize,
            mut a: [usize; n],
        }

        // Solve
        let mut ans = INF;

        for bit in 0..(1 << n) {
                let mut cost = 0;

                if (bit as u64).count_ones() as usize != k {
                        continue;
                }

                let mut h = 0;

                for i in 0..n {
                        if bit & (1 << i) != 0 {
                                if h >= a[i] {
                                        cost += (h + 1) - a[i];
                                        h = h + 1;
                                } else {
                                        h = a[i];
                                }
                        } else {
                                h = std::cmp::max(h, a[i]);
                        }
                }

                ans = std::cmp::min(ans, cost);
        }

        // Output
        println!("{}", ans);
}
