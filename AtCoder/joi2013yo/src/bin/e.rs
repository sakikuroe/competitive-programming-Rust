use proconio::input;

fn compress(v1: &mut Vec<usize>, v2: &mut Vec<usize>) -> Vec<usize> {
        let mut vs: Vec<usize> = vec![];
        for a in v1.into_iter() {
                vs.push(*a);
                vs.push(*a + 1);
        }
        for a in v2.into_iter() {
                vs.push(*a);
                vs.push(*a + 1);
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
                n: usize, m: usize,
                xydxyd_list: [(usize, usize, usize, usize, usize, usize); n],
        }

        // Initialize
        let mut x1_list: Vec<usize> = vec![];
        let mut y1_list: Vec<usize> = vec![];
        let mut d1_list: Vec<usize> = vec![];
        let mut x2_list: Vec<usize> = vec![];
        let mut y2_list: Vec<usize> = vec![];
        let mut d2_list: Vec<usize> = vec![];

        for &(x1, y1, d1, x2, y2, d2) in &xydxyd_list {
                x1_list.push(x1);
                y1_list.push(y1);
                d1_list.push(d1);
                x2_list.push(x2);
                y2_list.push(y2);
                d2_list.push(d2);
        }

        // Solve
        // 目盛り
        let xs: Vec<usize> = compress(&mut x1_list, &mut x2_list);
        let ys: Vec<usize> = compress(&mut y1_list, &mut y2_list);
        let ds: Vec<usize> = compress(&mut d1_list, &mut d2_list);

        // いもす法
        let mut imos: Vec<Vec<Vec<isize>>> = vec![vec![vec![0; ds.len()]; ys.len()]; xs.len()];
        // +1, -1の記録
        for i in 0..n {
                imos[x1_list[i]][y1_list[i]][d1_list[i]] += 1;
                imos[x1_list[i]][y2_list[i]][d2_list[i]] += 1;
                imos[x2_list[i]][y1_list[i]][d2_list[i]] += 1;
                imos[x2_list[i]][y2_list[i]][d1_list[i]] += 1;
                imos[x2_list[i]][y1_list[i]][d1_list[i]] -= 1;
                imos[x1_list[i]][y2_list[i]][d1_list[i]] -= 1;
                imos[x1_list[i]][y1_list[i]][d2_list[i]] -= 1;
                imos[x2_list[i]][y2_list[i]][d2_list[i]] -= 1;
        }

        // シミュレーション
        for i in 0..xs.len() {
                for j in 0..ys.len() {
                        for k in 0..ds.len() - 1 {
                                imos[i][j][k + 1] += imos[i][j][k];
                        }
                }
        }
        for i in 0..xs.len() {
                for j in 0..ys.len() - 1 {
                        for k in 0..ds.len() {
                                imos[i][j + 1][k] += imos[i][j][k];
                        }
                }
        }
        for i in 0..xs.len() - 1 {
                for j in 0..ys.len() {
                        for k in 0..ds.len() {
                                imos[i + 1][j][k] += imos[i][j][k];
                        }
                }
        }

        let ans: usize = {
                let mut ans: usize = 0;
                for i in 0..xs.len() {
                        for j in 0..ys.len() {
                                for k in 0..ds.len() {
                                        if imos[i][j][k] as usize >= m {
                                                ans += (xs[i + 1] - xs[i])
                                                        * (ys[j + 1] - ys[j])
                                                        * (ds[k + 1] - ds[k]);
                                        }
                                }
                        }
                }

                ans
        };

        // Output
        println!("{}", ans);
}
