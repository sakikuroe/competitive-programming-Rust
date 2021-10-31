use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
                xy_list: [(isize, isize); n],
        }

        // Solve
        let ans = {
                let mut res = 0;
                for i in 0..n {
                        for j in i + 1..n {
                                for k in j + 1..n {
                                        if (xy_list[i].0 - xy_list[k].0)
                                                * (xy_list[j].1 - xy_list[k].1)
                                                != (xy_list[j].0 - xy_list[k].0)
                                                        * (xy_list[i].1 - xy_list[k].1)
                                        {
                                                res += 1;
                                        }
                                }
                        }
                }
                res
        };

        // Output
        println!("{}", ans);
}
