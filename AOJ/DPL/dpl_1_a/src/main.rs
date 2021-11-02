use std::{cmp, collections::HashMap, io, usize::MAX};
// use proconio::input;

const INF: usize = MAX;

struct Memoization<'a> {
    c_list: &'a Vec<usize>,
    memo: HashMap<(usize, usize), usize>,
}

impl<'a> Memoization<'a> {
    fn new(data: &Vec<usize>) -> Memoization {
        Memoization {
            c_list: data,
            memo: HashMap::<(usize, usize), usize>::new(),
        }
    }

    // メモを行う関数
    fn memoization(&mut self, (i, j): (usize, usize)) -> usize {
        match self.memo.get(&(i, j)) {
            Some(&value) => return value,
            None => {
                let value = self.dp((i, j));
                self.memo.insert((i, j), value);
                return value;
            }
        }
    }

    // dp[i][j] := i番目までのコインでj円を支払うときの、コインの最小の枚数
    fn dp(&mut self, (i, j): (usize, usize)) -> usize {
        if j == 0 {
            return 0;
        } else if i == 0 {
            return INF;
        }

        if j >= self.c_list[i - 1] {
            cmp::min(
                self.memoization((i - 1, j)),
                self.memoization((i, j - self.c_list[i - 1])) + 1,
            )
        } else {
            self.dp((i - 1, j))
        }
    }
}

fn solve(n: usize, c_list: &Vec<usize>) -> usize {
    let m = c_list.len();
    let mut memo = Memoization::new(c_list);

    memo.dp((m, n))
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    // input! {
    //     n: usize, m: usize,
    //     c_list: [usize; m],
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();
    let _m: usize = word_list[1].parse::<usize>().unwrap();

    line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let c_list: Vec<usize> = word_list
        .into_iter()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // Solve
    let ans: usize = solve(n, &c_list);

    // Output
    output(ans);
}
