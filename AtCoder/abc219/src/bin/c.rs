use proconio::{marker::Chars, input};


fn atob(s: Vec<char>, x:Vec<char>, alpha:Vec<char>) -> Vec<char> {
    let mut res: Vec<char> = vec![];
    for i in s {
        for (j, &a) in x.iter().enumerate() {
            if i == a {
                res.push(alpha[j])
            }
        }
    }
    // println!("{:?}", res);
    res
}

fn btoa(s: Vec<char>,  x:Vec<char>, alpha:Vec<char>) -> Vec<char> {
    let mut res: Vec<char> = vec![];
    for i in s {
        for (j, &a) in alpha.iter().enumerate() {
            if i == a {
                res.push(x[j])
            }
        }
    }
    // KCprintln!("{:?}", res);
    res
}

fn main() {
    // Input
    input! {
        x: Chars,
        n: usize,
        s_list : [Chars; n],
    }
    let alpha = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    
    // Initialize
    let mut v: Vec<Vec<char>> = vec![];
    for s in s_list {
        v.push(atob(s,x.clone(), alpha.clone()));
    }

    v.sort();

    let mut ans: Vec<Vec<char>> = vec![];
    for i     in v {
        ans.push(btoa(i,x.clone(), alpha.clone()));
    }

    for i in ans {
        for c in i {
            print!("{}", c);
        }
        println!();
    }
   
    
    // Output

}
