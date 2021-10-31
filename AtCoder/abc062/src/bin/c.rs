use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                h: usize, w: usize,
        }

        // Solve
        let ans1 = {
                let mut res = if h % 3 == 0 {
                        0
                } else {
                        w
                };
                for i in 1..=h - 1 {
                        let s_list = vec![i * w, (h - i) * (w / 2), (h - i) * ((w + 1) / 2)];
                        res = cmp::min(
                                res,
                                s_list.iter().max().unwrap() - s_list.iter().min().unwrap(),
                        );
                }
                res
        };

        let (h, w) = (w, h);

        let ans2 = {
                let mut res = if h % 3 == 0 {
                        0
                } else {
                        w
                };
                for i in 1..=h - 1 {
                        let s_list = vec![i * w, (h - i) * (w / 2), (h - i) * ((w + 1) / 2)];
                        res = cmp::min(
                                res,
                                s_list.iter().max().unwrap() - s_list.iter().min().unwrap(),
                        );
                }
                res
        };

        let ans = cmp::min(ans1, ans2);

        // Output
        println!("{}", ans);
}
