use proconio::input;
use proconio::marker::Chars;
use std::cmp;

fn main() {
        // Input
        input! {
            s: Chars,
        }

        // Solve
        let n = s.len();
        let mut ans = 0;
        let mut cnt = 0;
        for i in 0..n {
                if s[i] == 'A' || s[i] == 'T' || s[i] == 'C' || s[i] == 'G' {
                        cnt += 1;
                } else {
                        cnt = 0;
                }
                ans = cmp::max(ans, cnt);
        }

        // Output
        println!("{}", ans);
}
