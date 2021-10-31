use proconio::input;
use std::usize::MAX;
const INF: usize = MAX / 3;

fn compress(v1: &mut Vec<usize>, v2: &mut Vec<usize>, size: usize) -> Vec<usize> {
        let mut vs: Vec<usize> = vec![];
        for a in v1.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct && (*a) as isize + direct <= size as isize {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        for a in v2.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct && (*a) as isize + direct <= size as isize {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        vs.sort();
        vs.dedup();
        for a in v1.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        for a in v2.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        vs
}

fn main() {
        // Input
        input! {
                n: usize,
                ab_list: [(usize, usize); n],
        }

        // Initialize
        let ab_list = ab_list.into_iter().map(|(a, b)| (a - 1, b)).collect::<Vec<(usize, usize)>>();

        // Solve
        let mut x1_list = vec![];
        let mut x2_list = vec![];
        for &(a, b) in &ab_list {
                x1_list.push(a);
                x2_list.push(a + b);
        }

        // 座標圧縮
        let xs: Vec<usize> = compress(&mut x1_list, &mut x2_list, INF);

        // imos法
        let mut imos: Vec<isize> = vec![0; xs.len()];
        // +1, -1チェック
        for i in 0..n {
                imos[x1_list[i]] += 1;
                imos[x2_list[i]] -= 1;
        }
        // シミュレーション
        for i in 0..xs.len() - 1 {
                imos[i + 1] += imos[i];
        }

        let mut memo = vec![0; 200001];
        for i in 0..xs.len() - 1 {
                memo[imos[i] as usize] += xs[i + 1] - xs[i];
        }

        // Output
        for i in 1..=n {
                println!("{}", memo[i])
        }
}
