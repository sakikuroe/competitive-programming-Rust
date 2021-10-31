use proconio::{marker::Chars, input};

fn main() {
    // Input
    input! {
        mut s1: Chars,
        mut s2: Chars,
        mut s3: Chars,
        t: Chars,
    }
    
    // Initialize
    
    // Solve
    let mut ans: Vec<char> = vec![];
    for c in t {
        match c {
            '1' => ans.append(&mut s1.clone()),
            '2' => ans.append(&mut s2.clone()),
            '3' => ans.append(&mut s3.clone()),
            _ => (),
        }
    }
    
    // Output
    for x in ans {
        print!("{}", x);
    }
    println!();

}
