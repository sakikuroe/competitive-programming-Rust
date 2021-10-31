use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
            n: usize,
            s: Chars,
        }

        // Initialize

        // Solve
        let mut ans = 1;
        let mut now = s[0];
        for i in 1..n {
                if now != s[i] {
                        now = s[i];
                        ans += 1;
                }
        }

        // Output
        println!("{}", ans);
}
