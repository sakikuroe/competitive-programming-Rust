use proconio::input;

fn main() {
        // Input
        input! {
                (txa,tya,txb,tyb,t,v) : (usize,usize,usize,usize,usize,usize),
                n: usize,
                xy_list: [(usize, usize);n ],
        }

        // Initialize

        // Solve
        let f = {
                let mut res = false;

                for i in 0..n {
                        let (x, y) = (xy_list[i].0 as f64, xy_list[i].1 as f64);

                        if ((txa as f64 - x) * (txa as f64 - x)
                                + (tya as f64 - y) * (tya as f64 - y))
                                .sqrt()
                                + ((txb as f64 - x) * (txb as f64 - x)
                                        + (tyb as f64 - y) * (tyb as f64 - y))
                                        .sqrt()
                                <= (t * v) as f64
                        {
                                res = true;
                                break;
                        }
                }

                res
        };
        let ans = if f {
                "YES"
        } else {
                "NO"
        };

        // Output
        println!("{}", ans);
}
