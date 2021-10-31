use proconio::input;

#[derive(Clone, Copy, Debug)]
struct Light {
        color: usize,
        length: usize,
}

fn main() {
        // Input
        input! {
                n: usize,
                l_list: [usize; n],
        }

        // Solve
        // 連続して交互が続いている個数を記録
        let v: Vec<Light> = {
                let mut v: Vec<Light> = vec![];
                for (i, l) in l_list.iter().enumerate() {
                        if i == 0 {
                                v.push(Light {
                                        color: *l,
                                        length: 1,
                                });
                                continue;
                        }
                        let tail = v.pop().unwrap();
                        if tail.color == *l {
                                v.push(tail);
                                v.push(Light {
                                        color: *l,
                                        length: 1,
                                });
                        } else {
                                v.push(Light {
                                        color: *l,
                                        length: tail.length + 1,
                                })
                        }
                }
                v
        };

        let ans = {
                let mut ans = 0;
                if v.len() > 2 {
                        for i in 0..v.len() - 2 {
                                ans = std::cmp::max(
                                        ans,
                                        v[i].length + v[i + 1].length + v[i + 2].length,
                                );
                        }
                } else {
                        for i in 0..v.len() {
                                ans += v[i].length;
                        }
                }

                ans
        };

        // Output
        println!("{}", ans);
}
