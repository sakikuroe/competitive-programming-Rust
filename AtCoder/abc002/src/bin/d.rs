use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

fn main() {
    // Input
    input! {
        n: usize, m: usize,
        xy: [(Usize1, Usize1); m],
    }

    // Solve
    let mut ans = vec![];
    let set = {
        let mut res = HashSet::new();
        for (x, y) in xy {
            res.insert((x, y));
        }
        res
    };

    for bit in 0..(1 << n) as usize  {
        let mut f = vec![];
        for i in 0..n {
            for j in i + 1..n {
                if bit & (1 << i) != 0 && bit & (1 << j) != 0 {
                    f.push(set.contains(&(i, j)));
                }
            }
        }
        if f.into_iter().fold(true, |x, y| x & y) {
            ans.push(bit.count_ones());
        }
    }

    // Output
    println!("{}", ans.into_iter().max().unwrap());
}
