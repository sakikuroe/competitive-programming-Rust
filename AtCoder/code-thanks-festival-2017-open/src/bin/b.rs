use proconio::{input, marker::Chars};

fn is_palindrome(s: &Vec<char>) -> bool {
        let length = s.len();
        for i in 0..length {
                if !(s[i] == 'X' || s[length - 1 - i] == 'X' || s[i] == s[(length - 1) - i]) {
                        return false;
                }
        }

        return true;
}

fn main() {
        // Input
        input! {
                mut s: Chars,
        }

        // Solve
        let ans = {
                // sが回文になるまで任意のchar 'X'を追加していく
                let mut cnt = 0;
                loop {
                        if is_palindrome(&s) {
                                break;
                        } else {
                                s.push('X');
                                cnt += 1;
                        };
                }

                cnt
        };

        // Output
        println!("{}", ans);
}
