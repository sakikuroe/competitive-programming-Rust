use proconio::input;
use std::cmp;

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                v_list: [isize; n],
        }

        // Solve
        let ans = {
                let mut res = 0;
                for left in 0..=k {
                        for right in 0..=k {
                                if left + right > cmp::min(k, n) {
                                        continue;
                                }

                                let mut hand = vec![];
                                for i in 0..right {
                                        hand.push(v_list[i]);
                                }
                                for i in 0..left {
                                        hand.push(v_list[n - 1 - i]);
                                }

                                hand.sort();

                                for i in 0..cmp::min(hand.len(), k - left - right) {
                                        hand[i] = cmp::max(hand[i], 0);
                                }

                                res = cmp::max(res, hand.iter().sum());
                        }
                }

                res
        };

        // Output
        println!("{}", ans);
}
