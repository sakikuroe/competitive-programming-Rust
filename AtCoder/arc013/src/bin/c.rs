use proconio::input;

fn main() {
        // Input
        input! {
                n: usize,
        }

        // Solve
        let mut grundy = 0; // grundyの単位元は0

        for _ in 0..n {
                input! {
                        x: usize, y: usize, z: usize,
                        m: usize,
                        xyz_list: [(usize, usize, usize); m],
                }
                let max_x = xyz_list.iter().map(|(x, _y, _z)| x).max().unwrap();
                let min_x = xyz_list.iter().map(|(x, _y, _z)| x).min().unwrap();
                let max_y = xyz_list.iter().map(|(_x, y, _z)| y).max().unwrap();
                let min_y = xyz_list.iter().map(|(_x, y, _z)| y).min().unwrap();
                let max_z = xyz_list.iter().map(|(_x, _y, z)| z).max().unwrap();
                let min_z = xyz_list.iter().map(|(_x, _y, z)| z).min().unwrap();

                grundy ^= min_x;
                grundy ^= x - (max_x + 1);
                grundy ^= min_y;
                grundy ^= y - (max_y + 1);
                grundy ^= min_z;
                grundy ^= z - (max_z + 1);
        }

        let ans = if grundy != 0 {
                "WIN"
        } else {
                "LOSE"
        };

        // Output
        println!("{}", ans);
}
