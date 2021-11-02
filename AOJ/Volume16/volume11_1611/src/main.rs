// use proconio::input;
use std::{cmp, io};

trait Dist {
    fn dist(&self) -> usize;
}

impl Dist for (usize, usize) {
    fn dist(&self) -> usize {
        return (self.0 as isize - self.1 as isize).abs() as usize;
    }
}

// dp(left, right) := [w_{left}, w_{right})から叩き出せるブロックの数の最大値
fn dp(left: usize, right: usize, w_list: &Vec<usize>, memo: &mut Vec<Vec<Option<usize>>>) -> usize {
    if let Some(value) = memo[left][right] {
        return value;
    }

    if right - left == 1 {
        return 0;
    }
    if right - left == 2 {
        return if (w_list[left], w_list[right - 1]).dist() <= 1 {
            2
        } else {
            0
        };
    }

    let res: usize = {
        if (w_list[left], w_list[right - 1]).dist() <= 1 {
            if dp(left + 1, right - 1, w_list, memo) == right - left - 2 {
                return right - left;
            }
        }

        let mut res = 0;
        for i in (left + 1)..=(right - 1) {
            res = cmp::max(res, dp(left, i, w_list, memo) + dp(i, right, w_list, memo))
        }

        res
    };

    memo[left][right] = Some(res);

    res
}

fn solve(n: usize, w_list: &Vec<usize>) -> usize {
    let mut memo: Vec<Vec<Option<usize>>> = vec![vec![None; 400]; 400];
    let ans = dp(0, n, w_list, &mut memo);
    ans
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    loop {
        // Input
            // input! {
            //     n: usize,
            // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let n: usize = word_list[0].parse::<usize>().unwrap();
        if n == 0 {
            break;
        }
            // input! {
            //     w_list: [usize; n],
            // }

        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let w_list: Vec<usize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list
                .into_iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        };

        // Solve
        let ans: usize = solve(n, &w_list);

        // Output
        output(ans);
    }
}
