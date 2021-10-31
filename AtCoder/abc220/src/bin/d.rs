use proconio::input;
const MOD: usize = 998244353;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let mut dp = vec![vec![0; 10]; n];
        dp[0][a_list[0]] = 1;

        for i in 0..n - 1 {
                for j in 0..=9 {
                        dp[i + 1][(a_list[i + 1] + j) % 10] += dp[i][j] % MOD;
                        dp[i + 1][(a_list[i + 1] * j) % 10] += dp[i][j] % MOD;
                }
        }

        // Output
        for i in 0..=9 {
                println!("{}", dp[n - 1][i] % MOD);
        }
}
