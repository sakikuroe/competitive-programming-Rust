use std::{cmp, io};
// use proconio::input;
// use proconio::marker::Chars;

struct Memoization<'a> {
    s1: &'a Vec<char>,
    s2: &'a Vec<char>,
    memo: Vec<Vec<Option<usize>>>,
}

impl<'a> Memoization<'a> {
    fn new(s1: &'a Vec<char>, s2: &'a Vec<char>) -> Memoization<'a> {
        Memoization {
            s1,
            s2,
            memo: vec![vec![None; 1001]; 1001],
        }
    }

    // メモを行う関数
    fn memoization(&mut self, (i, j): (usize, usize)) -> usize {
        match self.memo[i][j] {
            Some(value) => return value,
            None => {
                let value = self.dp((i, j));
                self.memo[i][j] = Some(value);
                return value;
            }
        }
    }

    // dp[i][j] := s1のi文字目とs2のj文字目までの最長共通部分列問題
    fn dp(&mut self, (i, j): (usize, usize)) -> usize {
        if i == 0 || j == 0 {
            return 0;
        }

        if self.s1[i - 1] == self.s2[j - 1] {
            return self.memoization((i - 1, j - 1)) + 1;
        } else {
            return cmp::max(self.memoization((i - 1, j)), self.memoization((i, j - 1)));
        }
    }
}

fn solve(s1: &Vec<char>, s2: &Vec<char>) -> usize {
    let mut memo = Memoization::new(s1, s2);

    memo.dp((s1.len(), s2.len()))
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    // input! {
    //     q: usize
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let q: usize = word_list[0].parse::<usize>().unwrap();

    for _ in 0..q {
        // Input2
        // input! {
        //     s1: Chars,
        //     s2: Chars,
        // }
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let s1 = word_list[0].chars().collect();

        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<&str> = line.split_whitespace().collect();
        let s2 = word_list[0].chars().collect();

        // Solve
        let ans: usize = solve(&s1, &s2);

        // Output
        output(ans);
    }
}