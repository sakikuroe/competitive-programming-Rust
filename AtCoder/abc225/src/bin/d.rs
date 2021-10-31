use proconio::input;
use proconio::marker::Isize1;

fn main() {
        // Input
        input! {
                n: usize, q: usize,
        }

        // Initialize
        let mut train = vec![(-1, -1); n];

        // Solve
        for _ in 0..q {
                input! {
                        f: usize,
                }
                if f == 1 {
                        input! {
                                x: Isize1, y: Isize1,
                        }
                        train[x as usize].1 = y;
                        train[y as usize].0 = x;
                } else if f == 2 {
                        input! {
                                x: Isize1, y: Isize1,
                        }
                        train[x as usize].1 = -1;
                        train[y as usize].0 = -1;
                } else {
                        input! {
                                x: Isize1,
                        }
                        let mut ans: Vec<usize> = vec![];
                        let mut u = x;
                        while train[u as usize].0 != -1 {
                                ans.push(train[u as usize].0 as usize);
                                u = train[u as usize].0;
                        }
                        ans.reverse();
                        ans.push(x as usize);
                        u = x;
                        while train[u as usize].1 != -1 {
                                ans.push(train[u as usize].1 as usize);
                                u = train[u as usize].1;
                        }
                        print!("{} ", ans.len());
                        for a in ans {
                                print!("{} ", a + 1);
                        }
                        println!();
                }
        }
}
