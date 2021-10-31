use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                s: Chars,
        }

        // Solve
        let ans = {
                let mut res = vec![0; s.len()];

                let mut lr = vec![0];

                let mut idx = 0;
                let mut now = 'R';
                for &c in &s {
                        if c == now {
                                lr[idx] += 1;
                        } else {
                                idx += 1;
                                lr.push(1);
                                now = c;
                        }
                }

                let mut cnt = 0;
                for i in 0..s.len() - 1 {
                        if s[i] == 'R' && s[i + 1] == 'L' {
                                res[i] += (lr[cnt] + 1) / 2 + lr[cnt + 1] / 2;
                                res[i + 1] += lr[cnt] / 2 + (lr[cnt + 1] + 1) / 2;

                                cnt += 2;
                        }
                }

                res
        };

        // Output
        for a in ans {
                print!("{} ", a);
        }
        println!();
}
