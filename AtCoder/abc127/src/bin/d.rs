use proconio::input;

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                mut a_list: [usize; n],
                mut bc_list: [(usize, usize); m],
        }

        // Initialize
        bc_list.sort_by_key(|x| x.1);
        bc_list.reverse();

        // Solve
        let ans = {
                let mut res = a_list;
                for (b, c) in bc_list {
                        for _ in 0..b {
                                res.push(c);
                        }
                        if res.len() > 2 * n {
                                break;
                        }
                }
                res.sort();
                res.reverse();

                let mut ans = 0;
                for i in 0..n {
                        ans += res[i];
                }

                ans
        };

        // Output
        println!("{}", ans);
}
