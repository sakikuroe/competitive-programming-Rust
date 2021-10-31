use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                mut a_list: [usize; n],
        }

        // Solve
        let ans_list = {
                let y1 = {
                        let mut res = a_list.iter().sum::<usize>();
                        for i in 0..n / 2 {
                                res -= a_list[2 * i + 1] * 2;
                        }

                        res
                };
                let mut res = vec![y1];

                for i in 0..n - 1 {
                        res.push(a_list[i] * 2 - res[i]);
                }

                res
        };

        // Output
        for ans in ans_list {
                println!("{}", ans);
        }
}
