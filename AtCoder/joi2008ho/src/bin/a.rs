use proconio::input;

#[derive(Clone, Copy, Debug)]
struct Stone {
        color: usize,
        length: usize,
}

fn main() {
        // Input
        input! {
            n: usize,
            c_list: [usize; n],
        }

        // Solve
        // Stoneに関するスタック
        // colorは碁石の色、lengthは同じ色の碁石が連続して並んでいる数を表す
        let mut v: Vec<Stone> = vec![];
        for (i, c) in c_list.iter().enumerate() {
                // 最初は一番先頭のの碁石を置く
                if i + 1 == 1 {
                        v.push(Stone {
                                color: *c,
                                length: 1,
                        });
                        continue;
                }

                // 奇数番目の碁石を置くとき
                if (i + 1) % 2 == 1 {
                        let tail = v.pop().unwrap();
                        if tail.color == *c {
                                v.push(Stone {
                                        color: tail.color,
                                        length: tail.length + 1,
                                });
                        } else {
                                v.push(tail);
                                v.push(Stone {
                                        color: *c,
                                        length: 1,
                                });
                        }
                }
                // 偶数番目の碁石を置くとき
                else {
                        let tail = v.pop().unwrap();
                        if tail.color == *c {
                                v.push(Stone {
                                        color: tail.color,
                                        length: tail.length + 1,
                                });
                        } else {
                                if v.len() == 0 {
                                        v.push(Stone {
                                                color: *c,
                                                length: 1 + tail.length,
                                        });
                                } else {
                                        let tail2 = v.pop().unwrap();
                                        v.push(Stone {
                                                color: *c,
                                                length: 1 + tail.length + tail2.length,
                                        });
                                }
                        }
                }
        }

        // vに関して、色が白(0)のものをカウントしていく
        let ans: usize = {
                let mut sum: usize = 0;
                for Stone {
                        color,
                        length,
                } in v
                {
                        if color == 0 {
                                sum += length
                        }
                }
                sum
        };

        // Output
        println!("{}", ans);
}
