use memoise::memoise;
use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;

#[memoise (0 <= i <= 1000, 0 <= j <= 1000)]
fn dp(i: usize, j: usize, h: usize, w: usize, a_mat: &Vec<Vec<char>>) -> usize {
        if i == h - 1 && j == w - 1 {
                return 1;
        }
        if a_mat[i][j] == '#' {
                return 0;
        }

        let mut res = 0;
        if h - 1 > i {
                res += dp(i + 1, j, h, w, a_mat)
        }
        if w - 1 > j {
                res += dp(i, j + 1, h, w, a_mat)
        }
        res % MOD
}

fn main() {
        // Input
        input! {
                h: usize, w: usize,
                a_mat: [Chars; h],
        }

        // Solve
        let ans = dp(0, 0, h, w, &a_mat);

        // Output
        println!("{}", ans % MOD);
}
