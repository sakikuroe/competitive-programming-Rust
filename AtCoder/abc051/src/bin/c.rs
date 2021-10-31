use proconio::input;

fn main() {
        // Input
        input! {
                (sx, sy, tx, ty): (isize, isize, isize, isize),
        }
        
        // Solve
        let ans_list = {
                let mut res: Vec<char> = vec![];

                for _ in 0..(ty - sy) {
                        res.push('U');
                }
                for _ in 0..(tx - sx) {
                        res.push('R');
                }

                for _ in 0..(ty - sy) {
                        res.push('D');
                }
                for _ in 0..(tx - sx) {
                        res.push('L');
                }

                res.push('L');

                for _ in 0..(ty - sy + 1) {
                        res.push('U');
                }
                for _ in 0..(tx - sx + 1) {
                        res.push('R');
                }

                res.push('D');
                res.push('R');


                for _ in 0..(ty - sy + 1) {
                        res.push('D');
                }
                for _ in 0..(tx - sx + 1) {
                        res.push('L');
                }

                res.push('U');

                res
        };
        
        // Output
        for ans in ans_list {
                print!("{}", ans);
        }
}
