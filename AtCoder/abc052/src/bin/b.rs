use proconio::{input, marker::Chars};

fn main() {
        // Input
        input! {
                n: usize,
                s: Chars,
        }

        // Solve
        let ans = {
                let mut v = vec![];

                let mut sum = 0;
                v.push(sum);
                for i in 0..n {
                        match s[i] {
                                'I' => sum += 1,
                                'D' => sum -= 1,
                                _ => (),
                        }
                        v.push(sum);
                }

                v.into_iter().max().unwrap()
        };

        // Output
        println!("{}", ans);
}
