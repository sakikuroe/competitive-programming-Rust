use proconio::{input, marker::Chars};
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
        input! {
                s: Chars,
                k: usize,
        }
        
        // Solve
        let mut bit = Bit::new(s.len());

        // Output
}

