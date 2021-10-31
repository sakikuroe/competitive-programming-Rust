use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                n: usize,
                a: [usize; n],
        }

        // Initialize
        let m = cmp::min(n, 8);

        // Solve
        let mut memo: Vec<Vec<usize>> = vec![vec![]; 200];
        for bit in 1..(1 << m) {
                let mut b = vec![];
                for i in 0..n {
                        if bit >> i & 1 == 1 {
                                b.push(i + 1);
                        }
                }

                let sum = b.iter().map(|&x| a[x - 1]).fold(0, |x, y| x + y) % 200;

                if memo[sum] != vec![] {
                        println!("Yes");
                        print!("{} ", (&memo[sum]).len());
                        for x in &memo[sum] {
                                print!("{} ", x);
                        }
                        println!();
                        print!("{} ", b.len());
                        for x in &b {
                                print!("{} ", x);
                        }
                        println!();
                        break;
                } else {
                        memo[sum] = b;
                }

                if bit == (1 << m) - 1 {
                        println!("No");
                }
        }
}
