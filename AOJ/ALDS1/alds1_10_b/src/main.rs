// use memoise::memoise;
use std::{cmp, io, usize::MAX};
// use proconio::input;

const INF: usize = MAX;

// #[memoise (0 <= left <= 99, 1 <= right <= 100)]
// dp(left, right) := (M_{left - 1}, M_{right - 1}]の計算に必要なスカラー乗算の最小の回数
fn dp(
    left: usize,
    right: usize,
    rc_list: &Vec<(usize, usize)>,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // 計算済みだったらメモを利用する
    if let Some(value) = memo[left][right] {
        return value;
    }

    // 初期条件（行列1つの場合）
    if right - left == 1 {
        memo[left][right] = Some(0);
        return 0;
    }

    let res: usize = {
        let mut res = INF;
        for i in (left + 1)..right {
            res = cmp::min(
                res,
                dp(left, i, rc_list, memo)
                    + dp(i, right, rc_list, memo)
                    + (rc_list[left].0) * (rc_list[i].0) * (rc_list[right - 1].1),
            );
        }

        res
    };

    memo[left][right] = Some(res);
    res
}

fn main() {
    // INPUT
    // input! {
    //     n: usize,
    //     rc_list : [(usize, usize); n],
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();

    let mut rc_list: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<usize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list
                .into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        };
        rc_list.push((word_list[0], word_list[1]));
    }

    // SOLVE
    let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; 100]; 100];
    let ans: usize = dp(0, n, &rc_list, &mut memo);

    // OUTPUT
    println!("{}", ans);
}
