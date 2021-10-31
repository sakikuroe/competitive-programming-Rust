use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                mut ab_list: [(usize, usize); n],
        }

        // Initialize
        ab_list.sort_by_key(|x| x.1);

        // Solve
        let f = {
                let mut res = true;
                let mut t = 0;
                for (a, b) in ab_list {
                        t += a;
                        if t > b {
                                res = false;
                        }
                }

                res
        };

        let ans = if f {
                "Yes"
        } else {
                "No"
        };

        // Output
        println!("{}", ans);
}
