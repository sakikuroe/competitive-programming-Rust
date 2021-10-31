use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                ab_list: [(usize, usize); n],
        }

        // Initialize

        // Solve
        let mut time_sum = {
                let mut res = 0.0;
                for (a, b) in ab_list.clone() {
                        res += (a as f64) / (b as f64);
                }

                res / 2.0
        };

        let mut ans = 0.0;

        for (a, b) in ab_list.clone() {
                if (a as f64) / (b as f64) <= time_sum {
                        time_sum -= (a as f64) / (b as f64);
                        ans += a as f64;
                } else {
                        ans += time_sum * (b as f64);
                        break;
                }
        }

        // Output
        println!("{}", ans);
}
