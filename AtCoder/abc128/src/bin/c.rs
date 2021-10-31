use proconio::input;
use proconio::marker::Usize1;

fn main() {
        // Input
        input! {
            n: usize, m: usize,
            s: [[Usize1]; m],
            p: [usize; m],
        }

        // Solve
        let mut ans = 0;
        for bit in 0..(1 << n) {
                let mut f = true;
                for i in 0..m {
                        let mut cnt = 0;
                        for &j in &s[i] {
                                if bit & (1 << j) != 0 {
                                        cnt += 1;
                                }
                        }
                        if p[i] != cnt % 2 {
                                f = false;
                        }
                }
                if f {
                        ans += 1;
                }
        }

        // Output
        println!("{}", ans);
}
