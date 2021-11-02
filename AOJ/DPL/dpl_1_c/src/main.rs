use std::{cmp, collections::HashMap, io};
// use proconio::input;

struct Goods {
    v: usize,
    w: usize,
}

struct Memoization<'a> {
    goods_list: &'a Vec<Goods>,
    memo: HashMap<(usize, usize), usize>,
}

impl<'a> Memoization<'a> {
    fn new(data: &Vec<Goods>) -> Memoization {
        Memoization {
            goods_list: data,
            memo: HashMap::<(usize, usize), usize>::new(),
        }
    }

    // メモを行う関数
    fn memoization(&mut self, (i, w_max): (usize, usize)) -> usize {
        match self.memo.get(&(i, w_max)) {
            Some(&value) => return value,
            None => {
                let value = self.knapsack((i, w_max));
                self.memo.insert((i, w_max), value);
                return value;
            }
        }
    }

    // i番目までの品物から重さの総和がw_max以下となるように選ぶときの価値の総和の最大値
    fn knapsack(&mut self, (i, w_max): (usize, usize)) -> usize {
        if i == 0 {
            return 0;
        }

        if w_max >= self.goods_list[i - 1].w {
            cmp::max(
                self.memoization((i, w_max - self.goods_list[i - 1].w))
                    + self.goods_list[i - 1].v,
                self.memoization((i - 1, w_max)),
            )
        } else {
            self.memoization((i - 1, w_max))
        }
    }
}

fn solve(w: usize, goods_list: &Vec<Goods>) -> usize {
    let n = goods_list.len();
    let mut memo = Memoization::new(goods_list);

    memo.knapsack((n, w))
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    // input! {
    //     n: usize, w: usize,
    //     vw_list: [(usize, usize); n],
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();
    let w: usize = word_list[1].parse::<usize>().unwrap();

    let mut vw_list: Vec<(usize, usize)> = Vec::new();
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
        vw_list.push((word_list[0], word_list[1]));
    }

    // Initialize
    let goods_list = vw_list
        .iter()
        .map(|&(v, w)| Goods { v, w })
        .collect::<Vec<Goods>>();

    // Solve
    let ans: usize = solve(w, &goods_list);

    // Output
    output(ans);
}