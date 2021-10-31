use proconio::input;
use std::cmp;
use std::collections::HashSet;
fn main() {
        // Input
        input! {
                n: usize, m: usize,
                xy_list: [(usize, usize); m],
        }

        // Solve
        let mut ans = 0;
        let xy_list = xy_list
                .into_iter()
                .map(|(x, y)| (x - 1, y - 1))
                .collect::<HashSet<(usize, usize)>>();

        for bit in 1..(1 << n) {
                let mut flag = true;
                for i in 0..n {
                        for j in i + 1..n {
                                if (bit >> i) & 1 == 1 && (bit >> j) & 1 == 1 {
                                        if !xy_list.contains(&(i, j)) {
                                                flag = false;
                                        }
                                }
                        }
                }
                if flag {
                        ans = cmp::max(ans, (bit as u64).count_ones());
                }
        }

        // Output
        println!("{}", ans);
}
