use memoise::memoise;
use proconio::input;
use std::collections::{HashMap, HashSet};
const MOD: usize = 10000;

fn solve(n: usize, determined: &HashMap<usize, usize>) -> usize {
    let res: usize = {
        let mut res = 0;

        for i in 1..=3 {
            for j in 1..=3 {
                res += dp(n, i, j, determined) % MOD
            }
        }

        res % MOD
    };

    res
}

#[memoise (2 <= n <= 100, 1 <= i <= 3, 1 <= j <= 3)]
// dp(n, i, j) := n日目を終えた時点で、.*jiとなっている予定の数
// 例えば、入力例 1において、
// dp(5, 1, 2) = #{12121, 13121} = 2
// dp(3, 1, 2) = #{131} = 1
// dp(5, 1, 3) = #{} = 0
fn dp(n: usize, i: usize, j: usize, determined: &HashMap<usize, usize>) -> usize {
    if n == 2 {
        let pair = (determined.get(&(n - 1)), determined.get(&n));
        return match pair {
            (Some(&b), Some(&a)) => {
                if i == a && j == b {
                    1
                } else {
                    0
                }
            }
            (Some(&b), None) => {
                if j == b {
                    1
                } else {
                    0
                }
            }
            (None, Some(&a)) => {
                if i == a {
                    1
                } else {
                    0
                }
            }
            (None, None) => 1,
        };
    }

    let today_pasta = {
        match determined.get(&n) {
            Some(&menu) => vec![menu].into_iter().collect::<HashSet<usize>>(),
            None => vec![1, 2, 3].into_iter().collect::<HashSet<usize>>(),
        }
    };

    let mut res = 0;

    // n 日目にiのパスタを食べるのならば、その方法について和を取る
    if today_pasta.contains(&i) {
        for k in 1..=3 {
            // 3日間続いていたら、遷移しない
            if i == k && k == j {
                continue;
            }

            res += dp(n - 1, j, k, determined) % MOD;
        }
    }

    res
}

fn main() {
    // Input
    input! {
        n: usize, k: usize,
        mut ab_list: [(usize, usize); k],
    }

    // Initialize
    let determined = {
        let mut determined = HashMap::<usize, usize>::new();
        for &(a, b) in &ab_list {
            determined.insert(a, b);
        }
        determined
    };

    // Solve
    let ans: usize = solve(n, &determined);

    // Output
    println!("{}", ans);
}
