use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let boxes = {
                let mut res = vec![0; n];

                for i in 0..n {
                        let mut p = 0;
                        let mut idx = n - 1 - i;
                        while idx + 1 <= n {
                                p += res[idx];
                                p %= 2;

                                idx += n - i;
                        }

                        if p != a_list[n - 1 - i] {
                                res[n - 1 - i] = 1;
                        } else {
                                res[n - 1 - i] = 0;
                        }
                }

                res
        };

        // Output
        println!("{}", boxes.iter().sum::<usize>());
        for i in 0..n {
                if boxes[i] == 1 {
                        print!("{}", i + 1);
                }
                println!();
        }
}
