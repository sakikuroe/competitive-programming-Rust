use proconio::input;

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
                q: usize,
                tx_list: [(usize, usize); q],
        }

        // Initialize
        let mut bit = Bit::new(200_000);

        // Solve
        let ans = {
                let mut res: Vec<usize> = vec![];
                for (t, x) in tx_list {
                        if t == 1 {
                                bit.add(x, 1);
                        } else {
                                let idx = bit.lower_bound(x as isize);
                                res.push(idx);
                                bit.add(idx, -1);
                        }
                }

                res
        };

        // Output
        for a in ans {
                println!("{}", a);
        }
}
