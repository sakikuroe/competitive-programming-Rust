use proconio::input;
use std::isize::MAX;
const INF: isize = MAX / 3;
use std::cmp;

fn main() {
        // Input
        input! {
                t: usize,
                rgb_list:[(usize, usize, usize);t],
        }

        // Initialize
        let mut ans_list:Vec<isize> = vec![];

        // Solve
        for (r,g,b) in rgb_list {
                let mut ans = INF;
                let mut v = vec![r,g,b];
                v.sort();
                let (r,g,b) = (v[0],v[1],v[2]);
                if (b - g) % 3 == 0 {
                                ans = cmp::min(ans, b as isize );
                }
                if (g - r) % 3 == 0 {
                                ans = cmp::min(ans, g as isize );
                }
                if (b - r) % 3 == 0 {
                                ans = cmp::min(ans, b as isize );
                }

                if ans == INF {
                        ans_list.push(-1);
                } else {
                        ans_list.push(ans);
                }
        }

        // Output
        for i in 0..t{
                println!("{}",ans_list[i]);
        }
}
