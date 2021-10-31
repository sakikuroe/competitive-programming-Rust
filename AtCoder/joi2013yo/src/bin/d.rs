use memoise::memoise;
use proconio::input;
use std::cmp;

trait Dist {
        fn dist(&self) -> usize;
}

impl Dist for (usize, usize) {
        fn dist(&self) -> usize {
                return (self.0 as isize - self.1 as isize).abs() as usize;
        }
}

#[memoise(0 <= i <= 199, 0 <= t <= 1)]
// dp(i - 1, 0) := i日目に派手さが最小の服を着るときにおける、派手さの総和の最大値
// dp(i - 1, 1) := i日目に派手さが最大の服を着るときにおける、派手さの総和の最大値
fn dp(i: usize, t: usize, min_list: &Vec<usize>, max_list: &Vec<usize>) -> usize {
        if i == 0 {
                return 0;
        }
        if t == 0 {
                cmp::max(
                        dp(i - 1, 0, min_list, max_list) + (min_list[i], min_list[i - 1]).dist(),
                        dp(i - 1, 1, min_list, max_list) + (min_list[i], max_list[i - 1]).dist(),
                )
        } else {
                cmp::max(
                        dp(i - 1, 0, min_list, max_list) + (max_list[i], min_list[i - 1]).dist(),
                        dp(i - 1, 1, min_list, max_list) + (max_list[i], max_list[i - 1]).dist(),
                )
        }
}

fn solve(clothes_list: &Vec<Vec<usize>>) -> usize {
        // min_list[i - 1] := i日目(1 <= i <= d)に着ることができる最小の派手さ
        let min_list: Vec<usize> = {
                let mut min_list = vec![];
                for clothes in clothes_list {
                        min_list.push(clothes[0]);
                }
                min_list
        };

        // max_list[i - 1] := i日目(1 <= i <= d)に着ることができる最大の派手さ
        let max_list: Vec<usize> = {
                let mut max_list = vec![];
                for clothes in clothes_list {
                        max_list.push(clothes[clothes.len() - 1]);
                }
                max_list
        };

        let n = clothes_list.len();

        cmp::max(dp(n - 1, 1, &min_list, &max_list), dp(n - 1, 0, &min_list, &max_list))
}

fn main() {
        // Input
        input! {
            d: usize, n: usize,
            t_list: [usize; d],
            abc_list: [(usize, usize, usize); n],
        }

        // Initialize
        // clothes_list[i - 1] := i日目(1 <= i <= d)に着ることのできる服の派手さのリスト
        let clothes_list = {
                let mut clothes_list: Vec<Vec<usize>> = vec![vec![]; d];
                for (i, &t) in t_list.iter().enumerate() {
                        for &(a, b, c) in &abc_list {
                                if a <= t && t <= b {
                                        clothes_list[i].push(c);
                                }
                        }
                        clothes_list[i].sort();
                }
                clothes_list
        };

        // Solve
        let ans: usize = solve(&clothes_list);

        // Output
        println!("{}", ans);
}
