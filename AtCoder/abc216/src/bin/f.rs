use memoise::memoise;
use proconio::input;

const MOD: usize = 998244353;

#[memoise(0 <= i <= 5001, 0 <= j <= 5001)]
// dp(i, j) := 0~i番目のb_listの要素のうち、合計がj以下となるような組み合わせの数
fn dp(i: isize, j: isize, b_list: &Vec<isize>) -> usize {
        if i == 0 {
                return 1;
        }

        if j >= b_list[i as usize - 1] {
                return (dp(i - 1, j, b_list) % MOD
                        + dp(i - 1, j - b_list[i as usize - 1], b_list) % MOD)
                        % MOD;
        } else {
                return dp(i - 1, j, b_list) % MOD;
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [isize; n],
                b_list: [isize; n],
        }

        // Solve
        let mut ab_list = {
                let mut res = vec![];
                for i in 0..n {
                        res.push((a_list[i], b_list[i]));
                }
                res
        };

        ab_list.sort();

        let a_list = ab_list.iter().map(|(a, _)| *a).collect::<Vec<isize>>();
        let b_list = ab_list.iter().map(|(_, b)| *b).collect::<Vec<isize>>();

        let mut ans = 0;
        for i in 0..n {
                if a_list[i] >= b_list[i] {
                        // a_list[i]を使う場合で考えているので、b_list[i]も必ず使う
                        // b_list[0]~b_list[i-1] で、合計がa_list[i] - b_list[i]となるような組み合わせを求める
                        ans += dp(i as isize, a_list[i] - b_list[i], &b_list);
                }
        }

        // Output
        println!("{}", ans % MOD);
}
