use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
        }

        // Initialize
        let mut num_mat = vec![vec![0;9];9];

        // Solve
        for i in 1..=n {
            let a = i % 10;
            let b = {
                let mut res = i;
                while res >= 10 {
                    res /= 10;
                }
                res
            };
            if a == 0 || b == 0 {
                continue;
            }
            
            num_mat[a-1][b-1] += 1;
        }

        let mut ans = 0;
        for i in 0..9 {
            for j in 0..9 {
                ans += num_mat[i][j]*num_mat[j][i];
            }
        }

        // Output
        println!("{}", ans);
}
