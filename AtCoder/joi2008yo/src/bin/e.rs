use proconio::input;

fn main() {
        // Input
        input! {
            r: usize, c: usize,
            a: [[usize; c]; r],
        }

        // Solve
        let mut ans = 0;
        for bit in 0..(1 << r) {
                let mut cnt = 0;
                for i in 0..c {
                        let mut cnt2 = 0;
                        for j in 0..r {
                                if bit & (1 << j) != 0 {
                                        if a[j][i] == 1 {
                                                cnt2 += 1;
                                        }
                                } else {
                                        if a[j][i] == 0 {
                                                cnt2 += 1;
                                        }
                                }
                        }
                        cnt += std::cmp::max(cnt2, r - cnt2);
                }

                ans = std::cmp::max(ans, cnt);
        }

        // Output
        println!("{}", ans);
}
