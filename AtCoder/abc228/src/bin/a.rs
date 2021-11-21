use proconio::input;

fn main() {
        // Input
        input! {
            s: usize, mut t:usize, mut x: usize,
        }

        // Solve
        let ans;
        if s > t {
                t += 24;
        }
        if s <= x && x < t {
                ans = "Yes";
        } else {
                x += 24;
                if s <= x && x < t {
                        ans = "Yes";
                } else {
                        ans = "No";
                }
        }

        // Output
        println!("{}", ans);
}
