use memoise::memoise;
use proconio::input;

#[memoise(0 <= n <= 100000)]
fn dp(n: usize, a_list: &Vec<usize>) -> bool {
        for &a in a_list {
                if n >= a {
                        if dp(n - a, a_list) == false {
                                return true;
                        }
                }
        }

        false
}

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                a_list: [usize; n],
        }

        // stack overflow対策
        // 再帰回数が少ないようにメモを取る
        for i in 0..k {
                dp(i, &a_list);
        }

        // Output
        if dp(k, &a_list) {
                println!("First");
        } else {
                println!("Second");
        }
}
