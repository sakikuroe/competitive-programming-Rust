use proconio::input;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq)]
struct Goods {
        a: Reverse<usize>,
        b: usize,
}
impl Ord for Goods {
        fn cmp(&self, other: &Self) -> Ordering {
                self.a.cmp(&other.a)
        }
}
impl PartialOrd for Goods {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Goods {
        fn eq(&self, other: &Self) -> bool {
                self.a == other.a
        }
}

fn main() {
        // Input
        input! {
                n: usize, k: usize,
                ab_list: [(usize, usize); n],
        }

        // Solve
        let mut que: BinaryHeap<Goods> = {
                let mut que = BinaryHeap::new();
                for (a, b) in ab_list {
                        que.push(Goods {
                                a: Reverse(a),
                                b,
                        });
                }
                que
        };

        let mut t = 0 as usize;
        // 優先度付きキューを用いて、かかる時間が小さいものを取り出し、tに加算し、aをa+bに更新して、キューに戻す
        // k回繰り返す
        for _ in 0..k {
                if let Some(Goods {
                        a: Reverse(a),
                        b,
                }) = que.pop()
                {
                        t += a;
                        que.push(Goods {
                                a: Reverse(a + b),
                                b,
                        });
                }
        }

        // Output
        println!("{}", t);
}
