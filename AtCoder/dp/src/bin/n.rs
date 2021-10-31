use memoise::memoise;
use proconio::input;
use std::cmp;
use std::usize::MAX;
const INF: usize = MAX / 3;

#[memoise(0 <= left <= 400, 0 <= right <= 400)]
// dp(left, right) := [a_list_{left}, a_list_{right})を一つにするときの最小のコスト
fn dp(left: usize, right: usize, a_list: &Vec<usize>, sum_list: &Vec<usize>) -> usize {
        if right - left == 1 {
                return 0;
        }
        if right - left == 2 {
                return sum_list[right] - sum_list[left];
        }

        let mut res = INF;

        for i in left + 1..=right - 1 {
                res = cmp::min(
                        res,
                        dp(left, i, &a_list, &sum_list)
                                + dp(i, right, &a_list, &sum_list)
                                + sum_list[right]
                                - sum_list[left],
                );
        }
        res
}
fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        // 累積和
        let sum_list = {
                let mut res = vec![0; n + 1];
                for i in 0..n {
                        res[i + 1] = res[i] + a_list[i];
                }
                res
        };
        let ans: usize = dp(0, n, &a_list, &sum_list);

        // Output
        println!("{}", ans);
}
