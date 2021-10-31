use proconio::input;

fn main() {
    // Input
    input! {
        x: usize,
    }
    
    // Initialize
    
    // Solve
    if 0 <= x && x < 40 {
        println!("{}", 40 - x);
    }
    else if 40 <= x && x < 70 {
        println!("{}", 70 - x);
    }
    else if 70 <= x && x < 90 {
        println!("{}", 90 - x);
    }
    else {
        println!("expert");
    }
    
    // Output

}
