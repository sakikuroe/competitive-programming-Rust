use proconio::input;

fn main() {
        // Input
        input! {
            n: usize,
        }

        // Solve
        let ans = {
                let mut res = 0;
                for i in 1..=n {
                        if i % 2 == 0 {
                                continue;
                        }
                        let mut cnt = 0;
                        for j in 1..=i {
                                if i % j == 0 {
                                        cnt += 1;
                                }
                        }
                        if cnt == 8 {
                                res += 1;
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
