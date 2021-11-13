use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
            s: [usize; n],
        }

        // Initialize
        let mut ans = 0;
        for i in 0..n {
                let mut f = false;
                let s = s[i];
                for a in 1..=1000 {
                        for b in 1..=1000 {
                                if s == 4 * a * b + 3 * a + 3 * b {
                                        f = true;
                                }
                        }
                }
                if f {
                        ans += 1;
                }
        }

        println!("{}", n - ans);
        // Solve

        // Output
}
