use memoise::memoise;
use proconio::input;

#[memoise (0 <= n <= 2999, 0 <= m <= 2999)]
// dp[n][m] := n枚目(0-based)までのコインで表がm枚出る確率
fn dp(n: usize, m: usize, p_list: &Vec<f64>) -> f64 {
        if n + 1 < m {
                return 0.0;
        }

        if n == 0 {
                if m == 0 {
                        return 1.0 - p_list[n];
                } else if m == 1 {
                        return p_list[n];
                }
        }

        if m == 0 {
                return dp(n - 1, m, p_list) * (1.0 - p_list[n]);
        }

        return dp(n - 1, m - 1, p_list) * (p_list[n]) + dp(n - 1, m, p_list) * (1.0 - p_list[n]);
}

fn main() {
        // Input
        input! {
                n: usize,
                p_list: [f64; n],
        }

        // Solve
        let ans = {
                let mut res = 0.0;
                for i in n / 2 + 1..=n {
                        res += dp(n - 1, i, &p_list);
                }
                res
        };

        // Output
        println!("{}", ans);
}
