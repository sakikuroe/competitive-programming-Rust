use proconio::input;
use proconio::marker::Chars;

fn main() {
        // Input
        input! {
                n: usize,
                s: Chars,
        }
        
        // Solve
        let ans = {
                let mut ans = vec![];
                let mut f = true;

                if f {
                        ans = vec![];
                        ans.push('S');
                        ans.push('S');
                        for i in 0..n-1 {
                                if ans[i+1] == 'S' {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        }
                                } else {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        }               
                                }
                        }
                        if ans[0] == ans[n] {
                                if ans[0] == 'S' {
                                        if (s[0] == 'o' && ans[1] == ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] != ans[n - 1]) {
                                                f = false;
                                        }
                                } else {
                                        if (s[0] == 'o' && ans[1] != ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] == ans[n - 1]) {
                                                f = false;
                                        }
                                }
                        }
                }

                if f {
                        ans = vec![];
                        ans.push('S');
                        ans.push('W');
                        for i in 0..n-1 {
                                if ans[i+1] == 'S' {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        }
                                } else {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        }               
                                }
                        }
                        if ans[0] == ans[n] {
                                if ans[0] == 'S' {
                                        if (s[0] == 'o' && ans[1] == ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] != ans[n - 1]) {
                                                f = false;
                                        }
                                } else {
                                        if (s[0] == 'o' && ans[1] != ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] == ans[n - 1]) {
                                                f = false;
                                        }
                                }
                        }
                }

                if f {
                        ans = vec![];
                        ans.push('W');
                        ans.push('S');
                        for i in 0..n-1 {
                                if ans[i+1] == 'S' {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        }
                                } else {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        }               
                                }
                        }
                        if ans[0] == ans[n] {
                                if ans[0] == 'S' {
                                        if (s[0] == 'o' && ans[1] == ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] != ans[n - 1]) {
                                                f = false;
                                        }
                                } else {
                                        if (s[0] == 'o' && ans[1] != ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] == ans[n - 1]) {
                                                f = false;
                                        }
                                }
                        }
                }
                if f {
                        ans = vec![];
                        ans.push('W');
                        ans.push('W');
                        for i in 0..n-1 {
                                if ans[i+1] == 'S' {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        }
                                } else {
                                        if s[i+1] == 'o' {
                                                if ans[i] == 'S' {
                                                        ans.push('W');
                                                } else {
                                                        ans.push('S');
                                                }
                                        } else {
                                                if ans[i] == 'S' {
                                                        ans.push('S');
                                                } else {
                                                        ans.push('W');
                                                }
                                        }               
                                }
                        }
                        if ans[0] == ans[n] {
                                if ans[0] == 'S' {
                                        if (s[0] == 'o' && ans[1] == ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] != ans[n - 1]) {
                                                f = false;
                                        }
                                } else {
                                        if (s[0] == 'o' && ans[1] != ans[n - 1]) 
                                        || (s[0] == 'x' && ans[1] == ans[n - 1]) {
                                                f = false;
                                        }
                                }
                        }
                }

                if f {
                        ans = vec![];
                }

                ans
        };
        
        // Output
        if ans.len() != 0 {
                for i in 0..n {
                        print!("{}", ans[i]);
                }
                println!();
        } else {
                println!("-1");
        }
}
