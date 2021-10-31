use proconio::input;
use std::cmp;
const MOD: usize = 998244353;

fn mod_pow(n: usize, m: usize) -> usize {
        if m == 0 {
                1
        } else if m % 2 == 0 {
                mod_pow(n * n % MOD, m / 2) % MOD
        } else {
                n * mod_pow(n, m - 1) % MOD
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                d: usize,
        }

        // Solve
        let mut ans = 0;
        if d == 1 {
                ans += mod_pow(2, n);
                ans -= 2;
        } else {
                for i in 0..n - 1 {
                        let deep = cmp::min(d, n - 1 - i);
                        if deep * 2 < d {
                                break;
                        }

                        let mut add = 0;
                        add += (2 * deep - d + 1) * mod_pow(2, d - 2);
                        add %= MOD;

                        if deep == d {
                                add += 2 * mod_pow(2, d - 2);
                                add %= MOD;
                        }

                        ans += add * mod_pow(2, i);
                        ans %= MOD;
                }
        }

        // Output
        println!("{}", (ans * 2) % MOD);
}
