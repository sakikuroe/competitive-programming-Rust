use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
                x: usize,
        }

        // Solve
        let sum = a_list.iter().sum::<usize>();

        let mut ans = (x / sum) * n;
        let mut sum = sum * (x / sum);

        for a in a_list {
                if sum > x {
                        break;
                }

                sum += a;
                ans += 1;
        }

        // Output
        println!("{}", ans);
}
