use proconio::{input, marker::Usize1};

fn main() {
        // Input
        input! {
            n: usize, x: Usize1,
            a: [Usize1; n],
        }

        // Solve
        let mut f = vec![false; n];
        let mut i = x;
        while !f[i] {
                f[i] = true;
                i = a[i];
        }

        let mut ans = 0;
        for i in 0..n {
                if f[i] {
                        ans += 1;
                }
        }

        // Output
        println!("{}", ans);
}
