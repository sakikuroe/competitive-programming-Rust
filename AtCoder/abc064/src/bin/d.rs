use proconio::input;
use proconio::marker::Chars;
use std::cmp;

fn main() {
        // Input
        input! {
                _n: usize,
                mut s: Chars,
        }

        // Solve
        let ans = {
                let mut cnt = 0;
                let mut left = 0;
                for &c in &s {
                        if c == '(' {
                                cnt -= 1;
                        } else {
                                cnt += 1;
                                left = cmp::max(left, cnt);
                        }
                }

                let mut cnt = 0;
                let mut right = 0;
                s.reverse();
                for &c in &s {
                        if c == ')' {
                                cnt -= 1;
                        } else {
                                cnt += 1;
                                right = cmp::max(right, cnt);
                        }
                }
                s.reverse();

                let mut res = vec![];
                for _ in 0..left {
                        res.push('(');
                }
                for c in s {
                        res.push(c);
                }
                for _ in 0..right {
                        res.push(')');
                }

                res
        };

        // Output
        for c in ans {
                print!("{}", c);
        }
        println!();
}
