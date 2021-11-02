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
}

fn main() {
        // Input
        // input! {
        //         n: usize, q: usize,
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let q: usize = iter.next().unwrap().parse::<usize>().unwrap();

        // Initialize
        let mut bit = Bit::new(n);

        // Solve
        let ans = {
                let mut res = vec![];
                for _ in 0..q {
                        // input! {
                        //         a: usize,
                        // }
                        let mut line = String::new();
                        io::stdin().read_line(&mut line).unwrap();
                        let mut iter = line.split_whitespace();
                        let a: usize = iter.next().unwrap().parse::<usize>().unwrap();

                        if a == 0 {
                                // input! {
                                //         b:usize, c: usize, d: usize,
                                // }
                                let b: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                let c: usize = iter.next().unwrap().parse::<usize>().unwrap();
                                let d: usize = iter.next().unwrap().parse::<usize>().unwrap();

                                bit.add(b, d as isize);
                                bit.add(c + 1, -1 * d as isize);
                        } else {
                                // input! {
                                //         b:usize,
                                // }
                                let b: usize = iter.next().unwrap().parse::<usize>().unwrap();

                                res.push(bit.sum(b as usize))
                        }
                }
                res
        };

        // Output
        for a in ans {
                println!("{}", a);
        }
}
