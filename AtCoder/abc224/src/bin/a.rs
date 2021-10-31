use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                s: Chars,
        }

        // Initialize

        // Solve
        let ans = if s[s.len() - 1] == 'r' {
                "er"
        } else {
                "ist"
        };

        // Output
        println!("{}", ans);
}
