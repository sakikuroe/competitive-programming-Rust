use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let ans = {
                let mut cnt_list = vec![0; 100001];
                for a in a_list {
                        cnt_list[a] = 1;
                }

                let sum = cnt_list.iter().sum::<usize>();

                if sum % 2 == 0 {
                        sum - 1
                } else {
                        sum
                }
        };

        // Output
        println!("{}", ans);
}
