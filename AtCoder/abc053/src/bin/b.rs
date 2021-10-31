use proconio::input;
use proconio::marker::Chars;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

fn main() {
        // Input
        input! {
                s: Chars,
        }

        // Solve
        let ans = {
                let mut a = INF;
                let mut z = 0;
                for i in 0..s.len() {
                        match s[i] {
                                'A' => a = cmp::min(a, i),
                                'Z' => z = cmp::max(z, i),
                                _ => (),
                        }
                }

                z - a + 1
        };

        // Output
        println!("{}", ans);
}
