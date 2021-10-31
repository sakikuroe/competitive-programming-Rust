use proconio::input;

fn main() {
        // Input
        input! {
                a: usize, b: usize, c: usize,
        }


        // Solve
        let ans = {
                let mut res = -1;

                for i in a..=b {
                        if i % c == 0 {
                                res = i as isize;
                                break;
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
