use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize;n],
        }

        // Initialize

        // Solve
        let ans = if a_list.iter().sum::<usize>() % n == 0 {
                let mut res = 0;
                let h = a_list.iter().sum::<usize>() / n;
                let mut sum = 0;
                for i in 0..n - 1 {
                        sum += a_list[i];
                        if sum != h * (i + 1) {
                                res += 1;
                        }
                }
                res
        } else {
                -1
        };

        // Output
        println!("{}", ans);
}
