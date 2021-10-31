use memoise::memoise;
use proconio::input;
use std::{cmp, usize::MAX};

const INF: usize = MAX;

trait Op {
    fn extended_add(&self) -> usize;
}

impl Op for (usize, usize) {
    fn extended_add(&self) -> usize {
        if let Some(value) = self.0.checked_add(self.1) {
            return value;
        } else {
            return INF;
        }
    }
}

#[memoise (0 <= bit <= 65536, 0 <= v <= 16)]
// dp(bit, v) := bitを訪れる必要のあるVの部分集合として、vからbitのすべての頂点を巡って0に行くような(経路の最小値, 経路の総数)
fn dp(bit: usize, v: usize, v_size: usize, road_mat: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    // 初期条件
    if bit == 0 && v == 0 {
        return (0, 1);
    }

    let mut res: (usize, usize) = (INF, 0);

    // 経路の長さの最小値を計算
    for u in 0..v_size {
        // uに移動できなければ（bitにuが含まれていなければ）continue
        if (bit & 1 << u) == 0 {
            continue;
        }

        // 道が閉鎖されていればcontinue
        if road_mat[v][u].1 < (dp(bit & !(1 << u), u, v_size, road_mat).0 , road_mat[v][u].0).extended_add() {
            continue;
        }

        // u番目のフラグを消して、dpを計算
        res.0 = cmp::min(res.0, (dp(bit & !(1 << u), u, v_size, road_mat).0, road_mat[v][u].0).extended_add());
    }

    // 経路の総数を計算
    for u in 0..v_size {
        // uに移動できなければ（bitにuが含まれていなければ）continue
        if (bit & 1 << u) == 0 {
            continue;
        }

        // 道が閉鎖されていればcontinue
        if road_mat[v][u].1 < (dp(bit & !(1 << u), u, v_size, road_mat).0 , road_mat[v][u].0).extended_add() {
            continue;
        }

        // 経路の長さが最小ならば、経路の総数に加える
        if res.0 == (dp(bit & !(1 << u), u, v_size, road_mat).0, road_mat[v][u].0).extended_add() {
            res.1 += dp(bit & !(1 << u), u, v_size, road_mat).1
        }
    }

    res
}

fn solve(v_size: usize, road_mat: &Vec<Vec<(usize, usize)>>) -> (usize, usize) {
    // v:0からGのすべての頂点を巡って0に行くような(経路の最小値、経路の総数)を返す
    // 到達できない場合は(INF, 0)
    dp((1 << v_size) - 1, 0, v_size, road_mat)
}

fn output(ans: (usize, usize)) {
    if ans.0 != INF {
        println!("{} {}", ans.0, ans.1);
    } else {
        println!("IMPOSSIBLE");
    }
}

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        stdt_list: [(usize, usize, usize, usize); m],
    }

    // Initialize
    // road_mat[i][j] := (i -> j の距離, i -> j の閉鎖時間)
    // 街の番号を0-basedに変更する（問題文は1-based）
    let road_mat: Vec<Vec<(usize, usize)>> = {
        let mut road_mat: Vec<Vec<(usize, usize)>> = vec![vec![(INF, 0); n]; n];
        for &(s, t, d, time) in &stdt_list {
            // 無向グラフなので、i -> j, j -> iでデータを設定する。
            road_mat[s - 1][t - 1] = (d, time);
            road_mat[t - 1][s - 1] = (d, time);
        }
        road_mat
    };

    // Solve
    let ans: (usize, usize) = solve(n, &road_mat);

    // Output
    output(ans);
}
