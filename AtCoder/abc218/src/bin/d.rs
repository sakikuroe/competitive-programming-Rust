use proconio::input;
use std::collections::HashSet;

fn main() {
        // Input
        input! {
                n: usize,
                xy_list: [(usize, usize); n],
        }

        // Solve
        let v_set = xy_list.clone().into_iter().collect::<HashSet<(usize, usize)>>();
        let mut ans = 0;
        for i in 0..n {
                for j in 0..n {
                        let a = xy_list[i];
                        let b = xy_list[j];
                        if a.0 >= b.0 || a.1 >= b.1 {
                                continue;
                        }
                        if v_set.contains(&(a.0, b.1)) && v_set.contains(&(b.0, a.1)) {
                                ans += 1;
                        }
                }
        }

        // Output
        println!("{}", ans);
}
