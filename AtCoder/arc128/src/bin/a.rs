use proconio::input;
// fdsi fsadlfsjfsodajfed
fn main() {
        // Input
        input! {
                n: usize,
                a_list: [usize; n],
        }

        // Solve
        let mut ans_list = vec![0; n];
        for i in 0..n -1{
                if a_list[i + 1] < a_list[i] {
                        ans_list[i] ^= 1;
                        ans_list[i+1] ^= 1;
                }
        }
                
         
        // Output
        for i in 0..n {
                print!("{} ", ans_list[i]);
        }
        println!();
}
