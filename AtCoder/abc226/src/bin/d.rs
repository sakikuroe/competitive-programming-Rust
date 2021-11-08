use proconio::input;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, Debug)]
struct Slope {
        value: (i64, i64),
}
impl Ord for Slope {
        fn cmp(&self, other: &Self) -> Ordering {
                if self.value.0 < 0 || self.value.1 > 0 && self.value.0 == 0 {
                        if other.value.0 < 0 || other.value.1 > 0 && other.value.0 == 0 {
                                (self.value.1 * other.value.0).cmp(&(self.value.0 * other.value.1))
                        } else {
                                (self.value.0 * other.value.1).cmp(&(self.value.1 * other.value.0))
                        }
                } else {
                        if other.value.0 < 0 || other.value.1 > 0 && other.value.0 == 0 {
                                (self.value.0 * other.value.1).cmp(&(self.value.1 * other.value.0))
                        } else {
                                (self.value.1 * other.value.0).cmp(&(self.value.0 * other.value.1))
                        }
                }
        }
}
impl PartialOrd for Slope {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Slope {
        fn eq(&self, other: &Self) -> bool {
                self.value.1 * other.value.0 == self.value.0 * other.value.1
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                xy_list: [[i64; 2]; n],
        }

        // Solve
        let mut v: Vec<Slope> = vec![];

        for i in 0..n - 1 {
                for j in i + 1..n {
                        v.push(Slope {
                                value: ((xy_list[j][0] - xy_list[i][0], xy_list[j][1] - xy_list[i][1])),
                        });
                }
        }

        v.sort();
        v.dedup();

        // Output
        println!("{}", v.len() << 1);
}
