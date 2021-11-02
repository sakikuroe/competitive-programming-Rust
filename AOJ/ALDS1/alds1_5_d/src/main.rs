// use proconio::input;
use std::io;

#[derive(Debug)]
struct Bit {
        size: usize,
        bit: Vec<usize>,
}
impl Bit {
        fn new(n: usize) -> Bit {
                Bit {
                        size: n,
                        bit: vec![0; n + 1],
                }
        }
        fn sum(&self, mut idx: usize) -> usize {
                let mut res = 0;
                while idx > 0 {
                        res += self.bit[idx as usize];
                        idx -= (idx as isize & -(idx as isize)) as usize;
                }
                res
        }
        fn add(&mut self, mut idx: usize, a: usize) {
                while idx as usize <= self.size {
                        self.bit[idx as usize] += a;
                        idx += (idx as isize & -(idx as isize)) as usize;
                }
        }
}
use std::usize::MAX;
const INF: usize = MAX / 3;
fn compress(v: &mut Vec<usize>, max_size: usize) -> Vec<usize> {
        let mut vs = vec![];
        for a in v.into_iter() {
                for direct in vec![-1, 0, 1] {
                        if 0 <= (*a) as isize + direct
                                && (*a) as isize + direct <= max_size as isize
                        {
                                vs.push(((*a) as isize + direct) as usize);
                        }
                }
        }
        vs.sort();
        vs.dedup();
        for a in v.into_iter() {
                *a = vs.binary_search(a).unwrap();
        }
        vs
}
fn main() {
        // Input
        // input! {
        //         n: usize,
        //         a_list: [usize; n],
        // }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut word_list = line.split_whitespace();
        let n: usize = word_list.next().unwrap().parse::<usize>().unwrap();

        let a_list = {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                let mut word_list: Vec<usize> = {
                        let word_list: Vec<&str> = line.split_whitespace().collect();
                        word_list.into_iter().map(|s| s.parse::<usize>().unwrap()).collect()
                };
                compress(&mut word_list,INF);
                word_list
        };
        
        // Solve
        let mut bit = Bit::new(600600);
        let mut ans = 0;
        for i in 0..n {
                ans += (i as usize - bit.sum(a_list[i]+1)) as usize;
                bit.add(a_list[i]+1, 1);
        }

        // Output
        println!("{}", ans);
}
