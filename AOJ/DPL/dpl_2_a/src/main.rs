// use proconio::input;
use std::{cmp, io, usize::MAX};

const INF: usize = MAX;

trait Op {
    fn extended_add(&self) -> usize;
}

impl Op for (usize, usize) {
    fn extended_add(&self) -> usize {
        if let Some(value) = self.0.checked_add(self.1) {
            return value
        } else {
            return INF
        }
    }
}

// dp(bit, v) := bitを訪れる必要のあるVの部分集合として、vからbitのすべての頂点をめぐって0に行くような経路の最小値を返す
fn dp(
    bit: usize,
    v: usize,
    v_size: usize,
    dist_mat: &Vec<Vec<usize>>,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // メモがあったら利用
    if let Some(value) = memo[bit][v] {
        return value;
    }

    // 初期条件
    if bit == 0 && v == 0 {
        memo[bit][v] = Some(0);
        return 0;
    }

    let mut res: usize = INF;

    for u in 0..v_size {
        // uに移動できなければ（bitにuが含まれていなければ）continue
        if (bit & 1 << u) == 0 {
            continue;
        }


        // u番めのフラグを消して、dpを計算
        res = cmp::min(
            res,
            (dp(bit & !(1 << u), u, v_size, dist_mat, memo), dist_mat[v][u]).extended_add(),
        );
    }

    // メモの記録
    memo[bit][v] = Some(res);

    res
}

fn solve(v_size: usize, dist_mat: &Vec<Vec<usize>>) -> usize {
    let mut memo = vec![vec![None; v_size]; 1 << v_size];

    // v:0からGのすべての頂点をめぐって0に行くような経路の最小値を返す
    dp((1 << v_size) - 1, 0, v_size, dist_mat, &mut memo)
}

fn output(ans: usize) {
    if ans != INF {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}

fn main() {
    // Input
        // input! {
        //     v_size: usize,
        //     e_size: usize,
        //     std_list: [(usize, usize, usize); e_size],
        // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let v_size: usize = word_list[0].parse::<usize>().unwrap();
    let e_size: usize = word_list[1].parse::<usize>().unwrap();

    let mut std_list: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..e_size {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<usize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list
                .into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        };
        std_list.push((word_list[0], word_list[1], word_list[2]));
    }

    // Initialize
    // dist_mat[i][j] := i -> j の距離
    let dist_mat: Vec<Vec<usize>> = {
        let mut dist_mat: Vec<Vec<usize>> = vec![vec![INF; v_size]; v_size];
        for &(s, t, d) in &std_list {
            dist_mat[s][t] = d;
        }
        dist_mat
    };

    // Solve
    let ans: usize = solve(v_size, &dist_mat);

    // Output
    output(ans);
}
