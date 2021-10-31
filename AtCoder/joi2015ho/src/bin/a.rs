use proconio::input;
use std::cmp;

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        p_list: [usize; m],
        abc_list: [(usize, usize, usize); n - 1],
    }

    // Initialize
    // 都市の番号、鉄道の番号を0-basedに直す
    let p_list: Vec<usize> = p_list.into_iter().map(|x| x - 1).collect::<Vec<usize>>();

    // Solve
    // sum_list[i] := 鉄道i (0 <= i < N - 1)を利用する回数
    // いもす法によって求める
    let sum_list: Vec<isize> = {
        let mut sum_list: Vec<isize> = vec![0; n];
        // +1, -1の記録
        for i in 0..m - 1 {
            let (p1, p2) = (cmp::min(p_list[i], p_list[i + 1]), cmp::max(p_list[i], p_list[i + 1]));
            sum_list[p1] += 1;
            sum_list[p2] -= 1;
        }

        // シミュレーション
        for i in 0..n-1 {
            sum_list[i + 1] += sum_list[i];
        }

        sum_list
    };

    let ans_list: Vec<usize> = {
        let mut res: Vec<usize> = vec![];
        for (i, &(a, b, c)) in abc_list.iter().enumerate() {
            // きっぷとICカードで料金が安くなる方を計算
            let ans = cmp::min(a * sum_list[i] as usize, b * sum_list[i] as usize + c);
            res.push(ans);
        }

        res
    };

    // Output
    // 各区間の料金の和を出力
    println!("{}", ans_list.into_iter().sum::<usize>());
}
