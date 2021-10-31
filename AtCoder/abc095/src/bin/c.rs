use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

fn main() {
        // Input
        input! {
            a: usize, b: usize, c: usize, x: usize, y: usize,
        }

        // Solve
        let mut ans = INF;
        for i in 0..=100000 {
                let mut sum = 2 * c * i;
                if i < x {
                        sum += a * (x - i);
                }
                if i < y {
                        sum += b * (y - i);
                }
                ans = cmp::min(ans, sum);
        }

        // Output
        println!("{}", ans);
}
