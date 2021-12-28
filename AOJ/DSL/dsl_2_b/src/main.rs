// use proconio::input;
use std::io;
#[derive(Debug)]
struct Bit {
    size: usize,
    bit: Vec<isize>,
}
impl Bit {
    fn new(n: usize) -> Bit {
        Bit {
            size: n,
            bit: vec![0; n + 1],
        }
    }
    fn sum(&self, mut idx: usize) -> isize {
        let mut res = 0;
        while idx > 0 {
            res += self.bit[idx];
            idx -= (idx as isize & -(idx as isize)) as usize;
        }
        res
    }
    fn add(&mut self, mut idx: usize, a: isize) {
        while idx <= self.size {
            self.bit[idx] = self.bit[idx] + a;
            idx += (idx as isize & -(idx as isize)) as usize;
        }
    }
    fn lower_bound(&mut self, key: isize) -> usize {
        let mut left = -1;
        let mut right = self.size as isize;
        while right - left != 1 {
            let mid = ((left + right) / 2) as usize;
            if key <= self.sum(mid) {
                right = mid as isize;
            } else {
                left = mid as isize;
            }
        }
        right as usize
    }
}

fn main() {
    // Input
    // input! {
    //         n: usize, q: usize,
    //         cxy_list: [(usize, usize, usize); q],
    // }
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse::<usize>().unwrap();
    let q: usize = iter.next().unwrap().parse::<usize>().unwrap();

    let mut cxy_list: Vec<(usize, usize, usize)> = Vec::new();
    for _ in 0..q {
        line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let word_list: Vec<usize> = {
            let word_list: Vec<&str> = line.split_whitespace().collect();
            word_list.into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
        };
        cxy_list.push((word_list[0], word_list[1], word_list[2]));
    }

    // Solve
    let mut ans_list = vec![];

    let mut bit = Bit::new(n);
    for (c, x, y) in cxy_list {
        if c == 0 {
            bit.add(x, y as isize);
        } else {
            ans_list.push(bit.sum(y) - bit.sum(x - 1));
        }
    }

    // Output
    for ans in ans_list {
        println!("{}", ans);
    }
}
