use proconio::input;

fn main() {
        // Input
        input! {
                mut x: isize,
        }

        // Solve
        let ans = {
                let mut cnt = 0;

                cnt += x / 11 * 2;
                x -= (x / 11) * 11;

                if 0 < x && x <= 6 {
                        cnt += 1;
                } else if 6 < x {
                        cnt += 2;
                }

                cnt
        };

        // Output
        println!("{}", ans);
}
