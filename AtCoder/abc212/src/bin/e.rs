use proconio::input;
const MOD: usize = 998244353;

fn main() {
        // Input
        input! {
                n: usize, m: usize, k: usize,
                uv_list: [(usize, usize); m],
        }

        // Initialize
        // 0-basedに変更する
        let uv_list =
                uv_list.into_iter().map(|(a, b)| (a - 1, b - 1)).collect::<Vec<(usize, usize)>>();

        // Solve
        // 辺が無い頂点のリスト
        let edge_list: Vec<Vec<usize>> = {
                let mut res: Vec<Vec<usize>> = vec![vec![]; n];
                for i in 0..n {
                        res[i].push(i);
                }
                for (u, v) in uv_list {
                        res[u].push(v);
                        res[v].push(u);
                }
                res
        };

        // dp[i][j] := i日の旅でjに到達する方法
        let mut dp = vec![vec![0 as usize; n]; k + 1];
        dp[0][0] = 1;
        for i in 0..k {
                let sum = {
                        let mut res = 0;
                        for j in 0..n {
                                res += dp[i][j] % MOD;
                        }
                        res
                };

                for j in 0..n {
                        let not = {
                                let mut res = 0;
                                let edge = edge_list[j].clone();
                                for e in edge {
                                        res += dp[i][e] % MOD;
                                }
                                res
                        };
                        dp[i + 1][j] = (sum - not) % MOD;
                }
        }

        let ans = dp[k][0] % MOD;

        // Output
        println!("{}", ans);
}
