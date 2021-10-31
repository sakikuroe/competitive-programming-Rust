use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                a: usize,
                b: usize,
                c: usize,
                d: usize,
        }

        // Solve
        let ans = cmp::max(a * b, c * d);

        // Output
        println!("{}", ans);
}
