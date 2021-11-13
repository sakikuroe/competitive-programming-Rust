use proconio::input;

fn main() {
        // Input
        input! {
            n: usize, k: usize, a: usize,
        }

        // Initialize
        let mut ans = a;
        for i in 0..(k - 1) {
                ans += 1;
                if ans == n + 1 {
                        ans = 1;
                }
        }

        println!("{}", ans);

        // Solve

        // Output
}
