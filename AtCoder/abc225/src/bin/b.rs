use proconio::input;
use proconio::marker::Usize1;

fn main() {
        // Input
        input! {
            n: usize,
            ab_list: [(Usize1, Usize1); n - 1],
        }

        // Initialize
        let mut g: Vec<Vec<usize>> = vec![vec![]; n];
        for (a, b) in ab_list.clone() {
                g[a].push(b);
                g[b].push(a);
        }

        // Solve
        let mut ans = false;
        for i in 0..n {
                if g[i].len() == n - 1 {
                        ans = true;
                }
        }

        // Output
        if ans {
                println!("Yes");
        } else {
                println!("No");
        }
}
