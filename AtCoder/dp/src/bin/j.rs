use memoise::memoise;
use proconio::input;

#[memoise (0 <= x <= 300, 0 <= y <= 300, 0 <= z <= 300)]
fn dp(x: usize, y: usize, z: usize, n: usize) -> f64 {
        if x == 0 && y == 0 && z == 0 {
                return 0.0;
        }
        let mut res = 1.0 / (1.0 - ((n - x - y - z) as f64) / n as f64);
        if x > 0 {
                res += dp(x - 1, y, z, n) * (x as f64)
                        / n as f64
                        / (1.0 - ((n - x - y - z) as f64) / n as f64);
        }
        if y > 0 {
                res += dp(x + 1, y - 1, z, n) * (y as f64)
                        / n as f64
                        / (1.0 - ((n - x - y - z) as f64) / n as f64);
        }
        if z > 0 {
                res += dp(x, y + 1, z - 1, n) * (z as f64)
                        / n as f64
                        / (1.0 - ((n - x - y - z) as f64) / n as f64);
        }
        res
}

fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let x = a_list.clone().into_iter().filter(|&x| x == 1).collect::<Vec<usize>>().len();
        let y = a_list.clone().into_iter().filter(|&x| x == 2).collect::<Vec<usize>>().len();
        let z = a_list.clone().into_iter().filter(|&x| x == 3).collect::<Vec<usize>>().len();

        let ans: f64 = dp(x, y, z, n);

        // Output
        println!("{}", ans);
}
