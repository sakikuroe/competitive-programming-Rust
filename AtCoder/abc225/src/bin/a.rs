use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
            mut s: Chars,
        }

        // Initialize
        s.sort();
        s.dedup();

        // Solve
        let ans = match s.len() {
                1 => 1,
                2 => 3,
                3 => 6,
                _ => -1,
        };

        // Output
        println!("{}", ans);
}
