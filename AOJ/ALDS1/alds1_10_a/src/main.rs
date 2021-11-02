use std::{collections::HashMap, io};
// use proconio::input;

struct Memoization {
    memo: HashMap<usize, usize>,
}

impl Memoization {
    fn new() -> Memoization {
        Memoization {
            memo: HashMap::<usize, usize>::new(),
        }
    }

    // メモを行う関数
    fn memoization(&mut self, i: usize) -> usize {
        match self.memo.get(&i) {
            Some(&value) => return value,
            None => {
                let value = self.fibonacci(i);
                self.memo.insert(i, value);
                return value;
            }
        }
    }

    fn fibonacci(&mut self, n: usize) -> usize {
        match n {
            0 => 1,
            1 => 1,
            _ => self.memoization(n - 1) + self.memoization(n - 2),
        }
    }
}

fn output(ans: usize) {
    println!("{}", ans);
}

fn main() {
    // Input
    // input! {
    //     n: usize,
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let word_list: Vec<&str> = line.split_whitespace().collect();
    let n: usize = word_list[0].parse::<usize>().unwrap();

    // solve
    let mut memo = Memoization::new();
    let ans: usize = memo.fibonacci(n);

    // Output
    output(ans);
}
