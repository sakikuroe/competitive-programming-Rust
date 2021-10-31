use proconio::{input, marker::Chars};

fn main() {
        // Input
        input! {
                n: usize, m: usize,
                a_mat:[Chars; 2*n],
        }

        // Solve
        let ans = {
                let mut v = vec![(0 as usize, 0 as usize); 2 * n];
                for i in 0..2 * n {
                        v[i].0 = i + 1;
                }

                for i in 0..m {
                        v.sort_by_key(|&x| x.0);
                        v.reverse();
                        v.sort_by_key(|&x| x.1);
                        v.reverse();

                        for j in 0..n {
                                match (a_mat[v[2 * j].0 - 1][i], a_mat[v[2 * j + 1].0 - 1][i]) {
                                        ('G', 'C') | ('C', 'P') | ('P', 'G') => v[2 * j].1 += 1,
                                        ('C', 'G') | ('G', 'P') | ('P', 'C') => v[2 * j + 1].1 += 1,
                                        _ => (),
                                };
                        }
                }
                v.sort_by_key(|&x| x.0);
                v.reverse();
                v.sort_by_key(|&x| x.1);
                v.reverse();

                v
        };

        for i in 0..2 * n {
                println!("{}", ans[i].0);
        }

        // Output
}
