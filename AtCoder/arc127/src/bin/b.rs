use proconio::{marker::Chars, input};

fn main() {
        // Input
        input! {
                n: Chars,
        }
    
        // Initialize
        let mut dp:Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 15];2];16];
        dp[0][0][0] = 1;
        
        // Solve
        for i in 0..n.len() {
                for j in 0..10 {
                        dp[i+1][1][j] += dp[i][1][j]*9;
                        dp[i+1][1][j+1] += dp[i][1][j];

                        let ni: usize = n[i] as usize - '0' as usize;
                
                        if ni > 1 {
                                dp[i+1][1][j] += dp[i][0][j] * (ni -1);
                                dp[i+1][1][j+1] += dp[i][0][j];

                        } else 
                        if ni == 1 {
                                dp[i+1][1][j] += dp[i][0][j]* (ni -1);
                        }

                        if ni == 1 {
                                dp[i+1][0][j+1] += dp[i][0][j];

                        } else {
                                dp[i+1][0][j] += dp[i][0][j];
                        }

                }
        }

        let ans = {
                let mut res = 0;
                for i in 0..15 {
                        res += (dp[n.len()][0][i] + dp[n.len()][1][i]) * i;
                }
                res
        };
        
        // Output
        println!("{}", ans);
}

