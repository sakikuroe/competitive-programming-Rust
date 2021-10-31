use proconio::input;
use proconio::marker::Usize1;

fn main() {
        // Input
        input! {
                n: usize, mut k: usize,
                ab_list: [(Usize1, usize); n],
        }

        // Solve
        let ans = {
                let mut bucket = vec![0; 100000];
                for (a, b) in ab_list {
                        bucket[a] += b;
                }

                let mut res = 0;
                for i in 0..100000 {
                        if k > bucket[i] {
                                k -= bucket[i];
                        } else {
                                res = i + 1;
                                break;
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
