use proconio::input;
use std::cmp::Ordering;

#[derive(Clone, Copy, Eq, Debug)]
struct Point {
        p: (usize, usize),
}
impl Ord for Point {
        fn cmp(&self, other: &Self) -> Ordering {
                (self.p.1 * other.p.0).cmp(&(self.p.0 * other.p.1))
        }
}
impl PartialOrd for Point {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
        }
}
impl PartialEq for Point {
        fn eq(&self, other: &Self) -> bool {
                self.p.1 * other.p.0 == self.p.0 * other.p.1
        }
}

fn main() {
        // Input
        input! {
                n: usize,
                xy_list: [(usize, usize); n],
        }

        // Initialize
        let mut v: Vec<(Point, Point)> = vec![];
        for (x, y) in xy_list.clone() {
                v.push((
                        Point {
                                p: (x, y - 1),
                        },
                        Point {
                                p: (x - 1, y),
                        },
                ));
        }
        v.sort_by_key(|&x| x.1);

        // Solve
        let mut ans = 0;
        let mut now = Point {
                p: (2000000000, 0),
        };
        for i in 0..n {
                if now <= v[i].0 {
                        ans += 1;
                        now = v[i].1;
                }
        }

        // Output
        println!("{}", ans);
}
