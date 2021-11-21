use proconio::{input, marker::Usize1};

fn main() {
        // Input
        input! {
            n: usize, k: Usize1,
            p: [[usize; 3]; n],
        }

        // Solve
        let p: Vec<usize> = p.into_iter().map(|x| x.into_iter().sum()).collect();
        let mut base = p.clone();

        base.sort();
        base.reverse();
        let x = base[k];

        let mut ans = vec![];
        for p in p {
                if p + 300 >= x {
                        ans.push("Yes");
                } else {
                        ans.push("No");
                }
        }

        // Output
        for a in ans {
                println!("{}", a);
        }
}
