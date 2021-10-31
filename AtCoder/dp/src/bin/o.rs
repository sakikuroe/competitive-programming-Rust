use memoise::memoise;
use proconio::input;
const MOD: usize = 1_000_000_007;

#[memoise (0 <= bit <= 2097152)]
// dp(bit) := bitをペアがまだ成立していない女性の集合として、それらの女性で作ることのできるペアの数
fn dp(bit: usize, n: usize, a_mat: &Vec<Vec<usize>>) -> usize {
        // 初期条件
        if bit == 0 {
                return 1;
        }

        let mut res: usize = 0;

        // bitの本数から、今考えている男性が何人目(0-based)なのかを計算する。
        let i = n - (bit as u64).count_ones() as usize;

        // i番目の男性とj番目(0 <= j <= n - 1)の女性でペアを作る
        for j in 0..n {
                // j番目の女性にペアがいればcontinue
                if (bit & 1 << j) == 0 {
                        continue;
                }

                // bit.count_ones()番目の男性と、i番目の女性の仲が悪ければcontinue
                if a_mat[i][j] == 0 {
                        continue;
                }

                // i番めのフラグを消して、(bit.count_ones()番目の男性と、i番目の女性でペアを作って)dpを計算
                res += dp(bit & !(1 << j), n, a_mat);
                res %= MOD;
        }

        res % MOD
}

fn main() {
        // Input
        input! {
                n: usize,
                a_mat: [[usize; n]; n],
        }

        // Solve
        let ans = dp((1 << n) - 1, n, &a_mat);

        // Output
        println!("{}", ans);
}
