use memoise::memoise;
use proconio::input;
use std::cmp;

const MOD: usize = 1_000_000_007;
fn mod_sub(a: usize, b: usize) -> usize {
        if a >= b {
                a - b
        } else {
                a + (MOD - b)
        }
}

#[memoise (0 <= n <= 101, 0 <= m <= 100001)]
//sum[n][m] := 累積和
fn sum(n: usize, m: usize, a_list: &Vec<usize>) -> usize {
        if m == 0 {
                return 0;
        }

        (dp(n, m - 1, a_list) % MOD + sum(n, m - 1, a_list) % MOD) % MOD
}

#[memoise (0 <= n <= 101, 0 <= m <= 100001)]
// n番目(0-based)までの子供たちでm個の飴を分け合う方法
fn dp(n: usize, m: usize, a_list: &Vec<usize>) -> usize {
        if n == 0 {
                if m <= a_list[n] {
                        return 1;
                } else {
                        return 0;
                };
        }

        mod_sub(
                sum(n - 1, m + 1, a_list) % MOD,
                sum(n - 1, cmp::max(0, m as isize - a_list[n] as isize) as usize, a_list) % MOD,
        ) % MOD
}

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                a_list: [usize; n],
        }

        // Solve
        for i in 0..n {
                for j in 0..k {
                        dp(i, j, &a_list);
                }
        }

        let ans = dp(n - 1, k, &a_list);

        // Output
        println!("{}", ans % MOD);
}
