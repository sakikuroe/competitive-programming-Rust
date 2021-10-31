use memoise::memoise;
use proconio::{input, marker::Chars};
use std::cmp;

#[memoise (0 <= n <= 2999, 0 <= m <= 2999)]
// dp[n][m] := sのn番目(0-based)、tのm番目までの文字列における解
fn dp(n: usize, m: usize, s: &Vec<char>, t: &Vec<char>) -> usize {
        if n == 0 && m == 0 {
                if s[n] == t[m] {
                        return 1;
                } else {
                        return 0;
                }
        }

        if n == 0 {
                return cmp::max(
                        dp(n, m - 1, s, t),
                        if s[n] == t[m] {
                                1
                        } else {
                                0
                        },
                );
        }

        if m == 0 {
                return cmp::max(
                        dp(n - 1, m, s, t),
                        if s[n] == t[m] {
                                1
                        } else {
                                0
                        },
                );
        }

        return cmp::max(
                dp(n - 1, m - 1, s, t)
                        + if s[n] == t[m] {
                                1
                        } else {
                                0
                        },
                cmp::max(dp(n - 1, m, s, t), dp(n, m - 1, s, t)),
        );
}

fn main() {
        // Input
        input! {
                s: Chars,
                t: Chars,
        }

        // Solve
        let s = {
                let mut res = vec![];
                let (mut i, mut j) = ((s.len() - 1) as isize, (t.len() - 1) as isize);
                while i >= 0 && j >= 0 {
                        if s[i as usize] == t[j as usize] {
                                res.push(s[i as usize]);
                                i -= 1;
                                j -= 1;
                        } else if i >= 1
                                && dp((i - 1) as usize, j as usize, &s, &t)
                                        == dp(i as usize, j as usize, &s, &t)
                        {
                                i -= 1;
                        } else if j >= 1
                                && dp(i as usize, (j - 1) as usize, &s, &t)
                                        == dp(i as usize, j as usize, &s, &t)
                        {
                                j -= 1;
                        } else {
                                break;
                        }
                }
                res.into_iter().rev().collect::<String>()
        };

        // Output
        println!("{}", s);
}
